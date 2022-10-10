use crate::types::{
    config::ConsensusConfig,
    errors::ConsensusStateError::{self, AddBlockPartError, CreateSignedVoteError},
    messages::{BlockPartMessage, ConsensusMessageType, MessageInfo, ProposalMessage, VoteMessage},
    peer::internal_peerid,
    round_state::{RoundState, RULE_4, RULE_5, RULE_6, RULE_7, RULE_8},
};
use bytes::Bytes;

use async_trait::async_trait;

use kai_lib::crypto::{crypto::keccak256, signature::verify_signature};
use kai_types::{
    block::{Block, BlockId},
    block_operations::BlockOperations,
    consensus::{executor::BlockExecutor, state::LatestBlockState},
    errors::{AddVoteError, VoteError},
    evidence::EvidencePool,
    part_set::PartSet,
    peer::PeerId,
    priv_validator::PrivValidator,
    proposal::{proposal_sign_bytes, Proposal},
    round::RoundStep,
    timestamp,
    types::SignedMsgType,
    vote::Vote,
};
use log::debug;
use mockall::automock;
use std::{
    convert::TryInto,
    fmt::Debug,
    ops::{Add, Mul},
    sync::{Arc, Mutex},
    thread,
};
use tokio::sync::mpsc::{
    error::{SendError, TryRecvError},
    Receiver, Sender,
};

const MSG_QUEUE_SIZE: usize = 1000;

#[automock]
#[async_trait]
pub trait ConsensusState: Debug + Send + Sync + 'static {
    fn get_config(&self) -> Arc<ConsensusConfig>;
    fn get_state(&self) -> Arc<Box<dyn LatestBlockState>>;
    fn get_block_operations(&self) -> Arc<Box<dyn BlockOperations>>;
    fn get_block_exec(&self) -> Arc<Box<dyn BlockExecutor>>;
    fn get_evidence_pool(&self) -> Arc<Box<dyn EvidencePool>>;
    fn get_rs(&self) -> Option<RoundState>;

    fn update_to_state(&self, state: Arc<Box<dyn LatestBlockState>>);
    async fn send(&self, msg_info: MessageInfo) -> Result<(), SendError<MessageInfo>>;
    fn start(self: Arc<Self>) -> Result<(), Box<ConsensusStateError>>;
    fn stop(self: Arc<Self>) -> Result<(), ConsensusStateError>;
}

#[derive(Debug)]
pub struct ConsensusStateImpl {
    pub config: Arc<ConsensusConfig>,
    pub rs: Arc<Mutex<RoundState>>,
    pub msg_chan_sender: Sender<MessageInfo>,
    pub msg_chan_receiver: Mutex<Receiver<MessageInfo>>,
    pub state: Arc<Box<dyn LatestBlockState>>,
    pub priv_validator: Arc<Box<dyn PrivValidator>>,
    pub block_operations: Arc<Box<dyn BlockOperations>>,
    pub block_exec: Arc<Box<dyn BlockExecutor>>,
    pub evidence_pool: Arc<Box<dyn EvidencePool>>,
}

#[async_trait]
impl ConsensusState for ConsensusStateImpl {
    fn get_config(&self) -> Arc<ConsensusConfig> {
        self.config.clone()
    }
    fn get_state(&self) -> Arc<Box<dyn LatestBlockState>> {
        self.state.clone()
    }

    fn get_block_operations(&self) -> Arc<Box<dyn BlockOperations>> {
        self.block_operations.clone()
    }

    fn get_block_exec(&self) -> Arc<Box<dyn BlockExecutor>> {
        self.block_exec.clone()
    }

    fn get_evidence_pool(&self) -> Arc<Box<dyn EvidencePool>> {
        self.evidence_pool.clone()
    }

    fn get_rs(&self) -> Option<RoundState> {
        if let Ok(rs_guard) = self.rs.clone().lock() {
            let rs = rs_guard.clone();
            drop(rs_guard);
            Some(rs)
        } else {
            None
        }
    }

    async fn send(&self, msg_info: MessageInfo) -> Result<(), SendError<MessageInfo>> {
        self.clone().msg_chan_sender.send(msg_info).await
    }

    /// should be called from node instance, in a tokio Runtime
    fn start(self: Arc<Self>) -> Result<(), Box<ConsensusStateError>> {
        // TODO:

        // start processing messages
        let cs = self.clone();
        tokio::spawn(async move {
            cs.process_msg_chan();
        });

        // start new round
        if let Ok(rs_guard) = self.clone().rs.clone().lock() {
            let round = rs_guard.round;
            drop(rs_guard);
            self.clone().start_new_round(round);
            Ok(())
        } else {
            Err(Box::new(ConsensusStateError::LockFailed(
                "round state".to_owned(),
            )))
        }
    }

    fn stop(self: Arc<Self>) -> Result<(), ConsensusStateError> {
        // close msg_channel
        if let Ok(mut msg_chan_receiver) = self.msg_chan_receiver.lock() {
            msg_chan_receiver.close();
        }

        Ok(())
    }

    fn update_to_state(&self, state: Arc<Box<dyn LatestBlockState>>) {
        todo!()
    }
}

impl ConsensusStateImpl {
    pub fn new(
        config: ConsensusConfig,
        state: Arc<Box<dyn LatestBlockState>>,
        priv_validator: Arc<Box<dyn PrivValidator>>,
        block_operations: Arc<Box<dyn BlockOperations>>,
        block_exec: Arc<Box<dyn BlockExecutor>>,
        evidence_pool: Arc<Box<dyn EvidencePool>>,
    ) -> Self {
        let (msg_chan_sender, msg_chan_receiver) = tokio::sync::mpsc::channel(MSG_QUEUE_SIZE);

        Self {
            config: Arc::new(config),
            state: state.clone(),
            priv_validator: priv_validator,
            block_operations: block_operations,
            block_exec: block_exec,
            evidence_pool: evidence_pool,
            rs: Arc::new(Mutex::new(RoundState::new_default())),
            msg_chan_sender: msg_chan_sender,
            msg_chan_receiver: Mutex::new(msg_chan_receiver),
        }
    }

    async fn internal_send(
        self: Arc<Self>,
        msg_info: MessageInfo,
    ) -> Result<(), SendError<MessageInfo>> {
        self.clone().msg_chan_sender.send(msg_info).await
    }

    /// Process incoming messages by polling the receiver.
    /// This process runs until the message channel is closed.
    fn process_msg_chan(self: Arc<Self>) {
        loop {
            let cs = self.clone();

            // stop processing if channel is closed
            if cs.msg_chan_sender.is_closed() {
                log::info!("message channel is closed, stop processing messages");
                break;
            }

            let lock_rs = cs.msg_chan_receiver.lock();
            if lock_rs.is_err() {
                log::error!("try lock receiver poisoned, exit the message processing");
                break;
            }

            let mut msg_chan_rx = lock_rs.unwrap();
            let rs = msg_chan_rx.try_recv();
            // unlock msg_chan_rx
            drop(msg_chan_rx);

            match rs {
                Ok(msg_info) => {
                    let msg = msg_info.msg.clone();
                    match msg.as_ref() {
                        ConsensusMessageType::ProposalMessage(_msg) => {
                            log::debug!("set proposal: proposal={:?}", _msg.clone());
                            if let Err(e) = self.clone().set_proposal(_msg.clone()) {
                                log::error!(
                                    "set proposal failed: peerid={} msg={:?}, err={}",
                                    msg_info.peer_id,
                                    _msg.clone(),
                                    e
                                );
                            }
                        }
                        ConsensusMessageType::BlockPartMessage(_msg) => {
                            log::debug!("set block part: blockpart={:?}", _msg.clone());
                            if let Err(e) = self.clone().add_proposal_block_part(_msg.clone()) {
                                log::error!(
                                    "set block part failed: peerid={} msg={:?}, err={}",
                                    msg_info.peer_id,
                                    _msg.clone(),
                                    e
                                );
                            }
                        }
                        ConsensusMessageType::VoteMessage(_msg) => {
                            log::debug!("set vote: vote={:?}", _msg.clone());
                            if let Err(e) = self
                                .clone()
                                .add_vote(_msg.clone(), msg_info.peer_id.clone())
                            {
                                log::error!(
                                    "set vote failed: peerid={} msg={:?}, err={}",
                                    msg_info.peer_id,
                                    _msg.clone(),
                                    e
                                );
                            }
                        }
                        _ => {
                            log::error!("unknown msg type: type={:?}", msg);
                        }
                    };

                    self.clone().check_upon_rules(msg);
                }
                Err(e) => match e {
                    TryRecvError::Disconnected => {
                        // break loop when channel is closed
                        break;
                    }
                    TryRecvError::Empty => {
                        // no messages, continue the loop
                    }
                },
            }
        }
    }

    fn check_upon_rules(self: Arc<Self>, msg: Arc<ConsensusMessageType>) {
        let msg = msg.clone();
        match msg.as_ref() {
            ConsensusMessageType::ProposalMessage(_msg) => {
                log::debug!(
                    "no upon rules to check for proposal, they are checked in block part message",
                );
            }
            ConsensusMessageType::BlockPartMessage(_msg) => {
                log::debug!(
                    "checking upon rules for proposal block part: block_part={:?}",
                    _msg.clone()
                );

                if let Ok(mut rs_guard) = self.rs.clone().lock() {
                    let rs = rs_guard.clone();

                    if rs
                        .proposal_block_parts
                        .is_some_and(|pbp| pbp.is_completed())
                    {
                        let proposal = rs.proposal.clone().unwrap();
                        let proposal_block_id = proposal.clone().block_id.clone().unwrap();
                        let pbp = rs.proposal_block_parts.clone().unwrap();

                        // TODO: validate fully received proposal block

                        // checking rule #2
                        if proposal.pol_round == 0 && rs.step == RoundStep::Propose {
                            if rs.locked_round == 0
                                || rs
                                    .locked_block_parts
                                    .is_some_and(|lb| lb.has_header(pbp.header()))
                            {
                                match self.clone().create_signed_vote(
                                    rs_guard.clone(),
                                    SignedMsgType::Prevote,
                                    proposal_block_id.clone(),
                                ) {
                                    Ok(signed_vote) => {
                                        let msg = MessageInfo {
                                            msg: Arc::new(ConsensusMessageType::VoteMessage(
                                                VoteMessage {
                                                    vote: Some(signed_vote),
                                                },
                                            )),
                                            peer_id: internal_peerid(),
                                        };

                                        self.msg_chan_sender
                                            .blocking_send(msg)
                                            .expect("send vote failed"); // should panics here?

                                        log::debug!("signed and sent vote")
                                    }
                                    Err(reason) => {
                                        debug!("create signed vote failed, reason: {:?}", reason);
                                    }
                                };
                            } else {
                                match self.clone().create_signed_vote(
                                    rs_guard.clone(),
                                    SignedMsgType::Prevote,
                                    BlockId::new_zero_block_id(), // nil block
                                ) {
                                    Ok(signed_vote) => {
                                        let msg = MessageInfo {
                                            msg: Arc::new(ConsensusMessageType::VoteMessage(
                                                VoteMessage {
                                                    vote: Some(signed_vote),
                                                },
                                            )),
                                            peer_id: internal_peerid(),
                                        };

                                        self.msg_chan_sender
                                            .blocking_send(msg)
                                            .expect("send vote failed"); // should panics here?

                                        log::debug!("signed and sent vote")
                                    }
                                    Err(reason) => {
                                        debug!("create signed vote failed, reason: {:?}", reason);
                                    }
                                };
                            }
                            rs_guard.step = RoundStep::Prevote;
                            drop(rs_guard);
                            return;
                        }

                        // checking rule #3
                        if proposal.pol_round > 0
                            && proposal.pol_round < rs.round
                            && rs.step == RoundStep::Propose
                            && rs
                                .clone()
                                .votes
                                .expect("vote should not nil")
                                .prevotes(proposal.pol_round)
                                .expect("prevotes should not nil")
                                .two_thirds_majority()
                                .is_some_and(|bid| bid.eq(&proposal_block_id))
                        {
                            if rs.locked_round <= proposal.pol_round
                                || rs
                                    .locked_block_parts
                                    .is_some_and(|lb| lb.has_header(pbp.header()))
                            {
                                match self.clone().create_signed_vote(
                                    rs_guard.clone(),
                                    SignedMsgType::Prevote,
                                    proposal_block_id.clone(),
                                ) {
                                    Ok(signed_vote) => {
                                        let msg = MessageInfo {
                                            msg: Arc::new(ConsensusMessageType::VoteMessage(
                                                VoteMessage {
                                                    vote: Some(signed_vote),
                                                },
                                            )),
                                            peer_id: internal_peerid(),
                                        };

                                        self.msg_chan_sender
                                            .blocking_send(msg)
                                            .expect("send vote failed"); // should panics here?

                                        log::debug!("signed and sent vote")
                                    }
                                    Err(reason) => {
                                        debug!("create signed vote failed, reason: {:?}", reason);
                                    }
                                };
                            } else {
                                match self.clone().create_signed_vote(
                                    rs_guard.clone(),
                                    SignedMsgType::Prevote,
                                    BlockId::new_zero_block_id(), // nil block
                                ) {
                                    Ok(signed_vote) => {
                                        let msg = MessageInfo {
                                            msg: Arc::new(ConsensusMessageType::VoteMessage(
                                                VoteMessage {
                                                    vote: Some(signed_vote),
                                                },
                                            )),
                                            peer_id: internal_peerid(),
                                        };

                                        self.msg_chan_sender
                                            .blocking_send(msg)
                                            .expect("send vote failed"); // should panics here?

                                        log::debug!("signed and sent vote")
                                    }
                                    Err(reason) => {
                                        debug!("create signed vote failed, reason: {:?}", reason);
                                    }
                                };
                            }
                            rs_guard.step = RoundStep::Prevote;
                            drop(rs_guard);
                            return;
                        }

                        // checking rule #5
                        if rs.step >= RoundStep::Prevote
                            && rs
                                .clone()
                                .votes
                                .expect("vote should not nil")
                                .prevotes(rs.round)
                                .expect("prevotes should not nil")
                                .two_thirds_majority()
                                .is_some_and(|bid| bid.eq(&proposal_block_id))
                            && !rs.triggered_rules.contains(&RULE_5)
                        {
                            if rs.step == RoundStep::Prevote {
                                rs_guard.locked_round = proposal.round;
                                rs_guard.locked_block = rs.proposal_block.clone();
                                rs_guard.locked_block_parts = rs.proposal_block_parts.clone();

                                match self.clone().create_signed_vote(
                                    rs_guard.clone(),
                                    SignedMsgType::Precommit,
                                    proposal_block_id.clone(),
                                ) {
                                    Ok(signed_vote) => {
                                        let msg = MessageInfo {
                                            msg: Arc::new(ConsensusMessageType::VoteMessage(
                                                VoteMessage {
                                                    vote: Some(signed_vote),
                                                },
                                            )),
                                            peer_id: internal_peerid(),
                                        };

                                        self.msg_chan_sender
                                            .blocking_send(msg)
                                            .expect("send vote failed"); // should panics here?

                                        log::debug!("signed and sent vote")
                                    }
                                    Err(reason) => {
                                        debug!("create signed vote failed, reason: {:?}", reason);
                                    }
                                };
                                rs_guard.step = RoundStep::Precommit;
                            }
                            rs_guard.valid_round = proposal.round;
                            rs_guard.valid_block = rs.proposal_block.clone();
                            rs_guard.valid_block_parts = rs.proposal_block_parts.clone();
                            drop(rs_guard);
                            return;
                        }

                        // checking rule #8
                        if rs
                            .clone()
                            .votes
                            .expect("vote should not nil")
                            .precommits(rs.round)
                            .expect("precommits should not nil")
                            .two_thirds_majority()
                            .is_some_and(|bid| bid.eq(&proposal_block_id))
                            && self.clone().block_operations.height() < rs.height
                        {
                            let precommits = rs.votes.unwrap().precommits(rs.round).unwrap();
                            let seen_commit = precommits
                                .make_commit()
                                .expect("error on creating commit from precommits");

                            self.clone().block_operations.save_block(
                                rs.proposal_block.unwrap(),
                                rs.proposal_block_parts.unwrap(),
                                seen_commit,
                            );

                            rs_guard.height += 1;

                            rs_guard.locked_round = 0;
                            rs_guard.locked_block = None;
                            rs_guard.locked_block_parts = None;

                            rs_guard.valid_round = 0;
                            rs_guard.valid_block = None;
                            rs_guard.valid_block_parts = None;

                            self.start_new_round(0);
                            drop(rs_guard);
                            return;
                        }
                    }
                }
            }
            ConsensusMessageType::VoteMessage(_msg) => {
                log::debug!("checking upon rules for proposal: vote={:?}", _msg.clone());

                match _msg
                    .clone()
                    .vote
                    .expect("vote should not nil")
                    .r#type
                    .into()
                {
                    SignedMsgType::Prevote => {
                        if let Ok(mut rs_guard) = self.rs.clone().lock() {
                            let rs = rs_guard.clone();

                            if rs.step == RoundStep::Prevote {
                                // checking rule #4, #5, #6
                                if let Some(maj23_block_id) = rs
                                    .clone()
                                    .votes
                                    .clone()
                                    .expect("vote should not nil")
                                    .prevotes(rs.round)
                                    .expect("prevotes should not nil")
                                    .two_thirds_majority()
                                {
                                    // if rule has not bee n triggered
                                    if !rs_guard.triggered_rules.contains(&RULE_4) {
                                        self.clone().schedule_timeout(
                                            rs.height,
                                            rs.round,
                                            RoundStep::Prevote,
                                        );
                                        // mark rule has triggered
                                        rs_guard.triggered_rules.insert(RULE_4);
                                    }

                                    // check rule #5
                                    // TODO: validate proposal, only valid proposal can proceed
                                    if rs
                                        .clone()
                                        .proposal
                                        .clone()
                                        .and_then(|p| p.block_id)
                                        .is_some_and(|pbid| pbid.eq(&maj23_block_id))
                                        && self.clone().block_operations.height() == rs.height - 1
                                        && rs.step >= RoundStep::Prevote
                                        && !rs_guard.triggered_rules.contains(&RULE_8)
                                    {
                                        let proposal = rs.clone().proposal.unwrap();
                                        let proposal_block_id = proposal.block_id.unwrap();

                                        if rs.step == RoundStep::Prevote {
                                            rs_guard.locked_round = proposal.round;
                                            rs_guard.locked_block = rs.proposal_block.clone();
                                            rs_guard.locked_block_parts =
                                                rs.proposal_block_parts.clone();

                                            match self.clone().create_signed_vote(
                                                rs_guard.clone(),
                                                SignedMsgType::Precommit,
                                                proposal_block_id.clone(),
                                            ) {
                                                Ok(signed_vote) => {
                                                    let msg = MessageInfo {
                                                        msg: Arc::new(
                                                            ConsensusMessageType::VoteMessage(
                                                                VoteMessage {
                                                                    vote: Some(signed_vote),
                                                                },
                                                            ),
                                                        ),
                                                        peer_id: internal_peerid(),
                                                    };

                                                    self.msg_chan_sender
                                                        .blocking_send(msg)
                                                        .expect("send vote failed"); // should panics here?

                                                    log::debug!("signed and sent vote")
                                                }
                                                Err(reason) => {
                                                    debug!(
                                                        "create signed vote failed, reason: {:?}",
                                                        reason
                                                    );
                                                }
                                            };
                                            rs_guard.step = RoundStep::Precommit;
                                        }
                                        rs_guard.valid_round = proposal.round;
                                        rs_guard.valid_block = rs.proposal_block.clone();
                                        rs_guard.valid_block_parts =
                                            rs.proposal_block_parts.clone();

                                        rs_guard.triggered_rules.insert(RULE_8);
                                    }

                                    // check rule #6
                                    if !maj23_block_id.is_zero()
                                        && rs.step == RoundStep::Prevote
                                        && !rs_guard.triggered_rules.contains(&RULE_6)
                                    {
                                        match self.clone().create_signed_vote(
                                            rs_guard.clone(),
                                            SignedMsgType::Precommit,
                                            BlockId::new_zero_block_id().clone(),
                                        ) {
                                            Ok(signed_vote) => {
                                                let msg = MessageInfo {
                                                    msg: Arc::new(
                                                        ConsensusMessageType::VoteMessage(
                                                            VoteMessage {
                                                                vote: Some(signed_vote),
                                                            },
                                                        ),
                                                    ),
                                                    peer_id: internal_peerid(),
                                                };

                                                self.msg_chan_sender
                                                    .blocking_send(msg)
                                                    .expect("send vote failed"); // should panics here?

                                                log::debug!("signed and sent vote")
                                            }
                                            Err(reason) => {
                                                debug!(
                                                    "create signed vote failed, reason: {:?}",
                                                    reason
                                                );
                                            }
                                        };
                                        rs_guard.triggered_rules.insert(RULE_6);
                                        rs_guard.step = RoundStep::Precommit;
                                    }
                                }
                            }
                            drop(rs_guard);
                        } else {
                            log::trace!("lock round state failed")
                        }
                    }
                    SignedMsgType::Precommit => {
                        if let Ok(mut rs_guard) = self.rs.clone().lock() {
                            let rs = rs_guard.clone();

                            // checking rule #7 and partial rule #8
                            if let Some(maj23_block_id) = rs
                                .votes
                                .clone()
                                .expect("vote should not nil")
                                .precommits(rs.round)
                                .expect("precommits should not nil")
                                .two_thirds_majority()
                            {
                                // if rule #7 has not been triggered
                                if !rs.triggered_rules.contains(&RULE_7) {
                                    self.clone().schedule_timeout(
                                        rs.height,
                                        rs.round,
                                        RoundStep::Precommit,
                                    );
                                    // mark rule has triggered
                                    rs_guard.triggered_rules.insert(RULE_7);
                                }

                                // check rule #8
                                // TODO: validate proposal, only valid proposal can proceed
                                if rs
                                    .proposal
                                    .and_then(|p| p.block_id)
                                    .is_some_and(|pbid| pbid.eq(&maj23_block_id))
                                    && self.clone().block_operations.height() == rs.height - 1
                                {
                                    let precommits =
                                        rs.votes.clone().unwrap().precommits(rs.round).unwrap();
                                    let seen_commit = precommits
                                        .make_commit()
                                        .expect("error on creating commit from precommits");

                                    self.clone().block_operations.save_block(
                                        rs.proposal_block.unwrap(),
                                        rs.proposal_block_parts.unwrap(),
                                        seen_commit,
                                    );

                                    rs_guard.height += 1;

                                    rs_guard.locked_round = 0;
                                    rs_guard.locked_block = None;
                                    rs_guard.locked_block_parts = None;

                                    rs_guard.valid_round = 0;
                                    rs_guard.valid_block = None;
                                    rs_guard.valid_block_parts = None;

                                    self.start_new_round(0);
                                } else {
                                    log::debug!("skipped checking for rule #8 due to received +2/3 precommits of block other than our proposal block");
                                };
                            }
                            drop(rs_guard);
                        } else {
                            log::trace!("lock round state failed")
                        }
                    }
                    other => {
                        debug!("no upon rules checking on vote type: {:?}", other)
                    }
                }
            }
            _ => {} // ignore other types
        };
    }

    fn set_proposal(&self, msg: ProposalMessage) -> Result<(), ConsensusStateError> {
        match self.clone().get_rs() {
            None => Err(ConsensusStateError::LockFailed("round state".to_owned())),
            Some(rs) => {
                // check for existing proposal
                if rs.proposal.is_some() {
                    return Err(ConsensusStateError::AddProposalError(
                        "ignored proposal: already have proposal".to_owned(),
                    ));
                }

                // ignore proposal comes from different height, round
                if msg.proposal.is_none() {
                    return Err(ConsensusStateError::AddProposalError(
                        "invalid proposal, forgot to validate proposal?".to_owned(),
                    ));
                }
                let proposal = msg.proposal.unwrap();

                if proposal.height != rs.height || proposal.round != rs.round {
                    return Err(ConsensusStateError::AddProposalError(
                        "ignored invalid proposal: height or round mismatch with round state"
                            .to_owned(),
                    ));
                }

                if proposal.pol_round >= proposal.round {
                    return Err(ConsensusStateError::AddProposalError(
                        "ignored proposal: invalid pol_round".to_owned(),
                    ));
                }

                if rs.validators.is_none() {
                    return Err(ConsensusStateError::AddProposalError(
                        "proposal verification error: cannot get validator set".to_owned(),
                    ));
                }
                let mut validator_set = rs.validators.unwrap();

                let proposer = validator_set.get_proposer();
                if proposer.is_none() {
                    return Err(ConsensusStateError::AddProposalError(
                        "proposal verification error: cannot get proposer".to_owned(),
                    ));
                }

                let proposer_address = proposer.unwrap().address;

                let chain_id = self.state.clone().get_chain_id();
                let psb = proposal_sign_bytes(chain_id, proposal.clone());
                if psb.is_none() {
                    return Err(ConsensusStateError::AddProposalError(
                        "proposal verification error: cannot get proposal sign bytes".to_owned(),
                    ));
                }
                if !verify_signature(
                    proposer_address,
                    keccak256(psb.unwrap()),
                    Bytes::from(proposal.clone().signature),
                ) {
                    return Err(ConsensusStateError::AddProposalError(
                        "proposal verification error: wrong signature from proposer".to_owned(),
                    ));
                }

                if let Ok(mut rs_guard) = self.clone().rs.lock() {
                    // update validators
                    rs_guard.validators = Some(validator_set);
                    // set proposal
                    rs_guard.proposal = Some(proposal.clone());
                    // set proposal block, it's empty until all block parts received
                    rs_guard.proposal_block = None;
                    // create new proposal block parts
                    rs_guard.proposal_block_parts = Some(PartSet::new_part_set_from_header(
                        proposal.clone().block_id.unwrap().part_set_header.unwrap(),
                    ));
                } else {
                    return Err(ConsensusStateError::AddProposalError(
                        "add proposal failed: cannot lock on round state".to_owned(),
                    ));
                }

                Ok(())
            }
        }
    }

    fn add_proposal_block_part(&self, msg: BlockPartMessage) -> Result<(), ConsensusStateError> {
        match self.clone().get_rs() {
            None => Err(ConsensusStateError::LockFailed("round state".to_owned())),
            Some(rs) => {
                if rs.height != msg.height {
                    log::debug!(
                        "ignored block part from wrong height: height={}, round={}",
                        msg.height,
                        msg.round
                    );
                    return Ok(());
                }

                // we're not expecting a block part
                if rs.proposal_block_parts.is_none() {
                    log::info!(
                        "ignored a block part when we're not expecting any: height={} round={} index={}",
                        msg.height,
                        msg.round,
                        msg.part.unwrap().index
                    );
                    return Ok(());
                }

                if let Ok(mut rs_guard) = self.clone().rs.lock() {
                    let pbp = rs_guard.proposal_block_parts.as_mut().unwrap();
                    match pbp.add_part(msg.clone().part.unwrap()) {
                        Ok(_) => {
                            log::info!("set proposal block part: part={:?}", msg.clone());
                            return Ok(());
                        }
                        Err(e) => return Err(AddBlockPartError(e)),
                    }
                } else {
                    return Err(ConsensusStateError::AddBlockPartError(
                        "add proposal block part: cannot lock on round state".to_owned(),
                    ));
                }
            }
        }
    }

    fn add_vote(
        self: Arc<Self>,
        msg: VoteMessage,
        peer_id: PeerId,
    ) -> Result<(), ConsensusStateError> {
        match self.clone().get_rs() {
            None => Err(ConsensusStateError::LockFailed("round state".to_owned())),
            Some(rs) => {
                // this is safe, vote msg has been validated
                let vote = msg.vote.unwrap();

                // precommit vote of previous height
                if vote.height + 1 == rs.height && vote.r#type == SignedMsgType::Precommit.into() {
                    if rs.step > RoundStep::Propose {
                        log::debug!("ignored late precommit of previous height came after propose step of next height");
                        return Ok(());
                    }

                    match rs.last_commit {
                        None => {
                            return Err(ConsensusStateError::AddVoteError(
                                AddVoteError::InvalidVote(VoteError::NilVote),
                            ))
                        }
                        Some(mut last_commit) => {
                            if let Ok(mut rs_guard) = self.clone().rs.lock() {
                                if let Err(e) = last_commit.add_vote(vote.clone()) {
                                    log::error!("ignored adding vote due to: {}", e);
                                    return Err(ConsensusStateError::AddVoteError(e));
                                }

                                // update last commit with added vote
                                rs_guard.last_commit = Some(last_commit);
                                drop(rs_guard);
                                log::info!("addded to last precommits: {:?}", vote.clone());
                            } else {
                                return Err(ConsensusStateError::LockFailed(
                                    "round state".to_owned(),
                                ));
                            }
                            return Ok(());
                        }
                    }
                }

                // height mismatch is ignored.
                if vote.height != rs.height {
                    log::debug!(
                        "ignored vote due to height mismatch: voteHeight={} rsHeight={}",
                        vote.height,
                        rs.height
                    );
                    return Ok(());
                }

                match rs.votes.clone() {
                    None => {
                        return Err(ConsensusStateError::AddVoteError(
                            kai_types::errors::AddVoteError::InvalidVote(VoteError::NilVote),
                        ))
                    }
                    Some(mut votes) => {
                        if let Err(e) = votes.add_vote(vote, peer_id) {
                            return Err(ConsensusStateError::AddVoteError(e));
                        }

                        // update votes to state with added vote
                        if let Ok(mut rs_guard) = self.clone().rs.lock() {
                            rs_guard.votes = Some(votes);
                            drop(rs_guard);
                            return Ok(());
                        } else {
                            return Err(ConsensusStateError::LockFailed("round state".to_owned()));
                        }
                    }
                }
            }
        }
    }

    fn schedule_timeout(self: Arc<Self>, height: u64, round: u32, step: RoundStep) {
        match step {
            RoundStep::Propose => {
                let timeout_propose = self.clone().get_config().timeout_propose.clone();
                let timeout_propose_delta = self.clone().get_config().timeout_propose_delta.clone();
                tokio::spawn(async move {
                    let sleep_duration = if round > 1 {
                        timeout_propose.add(timeout_propose_delta.mul(round - 1))
                    } else {
                        timeout_propose
                    };
                    thread::sleep(sleep_duration);
                    self.clone().on_timeout_propose(height, round);
                });
            }
            RoundStep::Prevote => {
                let timeout_prevote = self.clone().get_config().timeout_prevote.clone();
                let timeout_prevote_delta = self.clone().get_config().timeout_prevote_delta.clone();
                tokio::spawn(async move {
                    let sleep_duration = if round > 1 {
                        timeout_prevote.add(timeout_prevote_delta.mul(round - 1))
                    } else {
                        timeout_prevote
                    };
                    thread::sleep(sleep_duration);
                    self.clone().on_timeout_prevote(height, round);
                });
            }
            RoundStep::Precommit => {
                let timeout_precommit = self.clone().get_config().timeout_precommit.clone();
                let timeout_precommit_delta =
                    self.clone().get_config().timeout_precommit_delta.clone();
                tokio::spawn(async move {
                    let sleep_duration = if round > 1 {
                        timeout_precommit.add(timeout_precommit_delta.mul(round - 1))
                    } else {
                        timeout_precommit
                    };
                    thread::sleep(sleep_duration);
                    self.clone().on_timeout_precommit(height, round);
                });
            }
            _ => {}
        }
    }

    fn on_timeout_propose(self: Arc<Self>, height: u64, round: u32) {
        if let Ok(mut rs_guard) = self.rs.clone().lock() {
            if height == rs_guard.height
                && round == rs_guard.round
                && rs_guard.step == RoundStep::Propose
            {
                log::debug!("propose timed out, sending prevote for nil");

                match self.clone().create_signed_vote(
                    rs_guard.clone(),
                    SignedMsgType::Prevote,
                    BlockId::new_zero_block_id(), // nil block
                ) {
                    Ok(signed_vote) => {
                        let msg = MessageInfo {
                            msg: Arc::new(ConsensusMessageType::VoteMessage(VoteMessage {
                                vote: Some(signed_vote),
                            })),
                            peer_id: internal_peerid(),
                        };

                        self.msg_chan_sender
                            .blocking_send(msg)
                            .expect("send vote failed"); // should panics here?

                        log::debug!("signed and sent vote")
                    }
                    Err(reason) => {
                        debug!("create signed vote failed, reason: {:?}", reason);
                    }
                };
            }
            rs_guard.step = RoundStep::Prevote;
            drop(rs_guard);
        } else {
            log::trace!("lock round state failed")
        }
    }

    fn on_timeout_prevote(self: Arc<Self>, height: u64, round: u32) {
        if let Ok(mut rs_guard) = self.rs.clone().lock() {
            if height == rs_guard.height
                && round == rs_guard.round
                && rs_guard.step == RoundStep::Prevote
            {
                log::debug!("prevote timed out, sending precommit for nil");

                match self.clone().create_signed_vote(
                    rs_guard.clone(),
                    SignedMsgType::Precommit,
                    BlockId::new_zero_block_id(), // nil block
                ) {
                    Ok(signed_vote) => {
                        let msg = MessageInfo {
                            msg: Arc::new(ConsensusMessageType::VoteMessage(VoteMessage {
                                vote: Some(signed_vote),
                            })),
                            peer_id: internal_peerid(),
                        };

                        self.msg_chan_sender
                            .blocking_send(msg)
                            .expect("send vote failed"); // should panics here?

                        log::debug!("signed and sent vote")
                    }
                    Err(reason) => {
                        debug!("create signed vote failed, reason: {:?}", reason);
                    }
                };
            }
            rs_guard.step = RoundStep::Precommit;
            drop(rs_guard);
        } else {
            log::trace!("lock round state failed")
        }
    }

    fn on_timeout_precommit(self: Arc<Self>, height: u64, round: u32) {
        if let Ok(rs_guard) = self.rs.clone().lock() {
            let rs = rs_guard.clone();
            drop(rs_guard);
            if height == rs.height && round == rs.round {
                self.start_new_round(rs.round + 1);
            }
        } else {
            log::trace!("lock round state failed")
        }
    }

    fn start_new_round(self: Arc<Self>, round: u32) {
        if let Ok(mut rs_guard) = self.rs.clone().lock() {
            rs_guard.round = round;
            rs_guard.step = RoundStep::Propose;

            // clear old proposal
            rs_guard.proposal = None;
            rs_guard.proposal_block = None;
            rs_guard.proposal_block_parts = None;
            // clear triggered rules
            rs_guard.triggered_rules.clear();
            let rs = rs_guard.clone();
            drop(rs_guard);

            if self.clone().is_proposer() {
                self.clone().decide_proposal(rs);
            } else {
                self.clone().schedule_timeout(rs.height, rs.round, rs.step);
            }
        } else {
            log::trace!("lock round state failed")
        }
    }

    fn decide_proposal(self: Arc<Self>, rs: RoundState) {
        let block: Option<Block>;
        let block_parts: Option<PartSet>;

        if rs.valid_block.is_some() {
            block = rs.valid_block;
            block_parts = rs.valid_block_parts;
        } else {
            (block, block_parts) = self.clone().create_proposal_block();
            if block.is_none() {
                log::trace!("create proposal block failed");
            }
        }

        if let (Some(b), Some(bp)) = (block, block_parts) {
            let hash = b.hash().expect("calculate hash of block failed");

            let block_id = BlockId {
                hash: hash.to_vec(),
                part_set_header: Some(bp.header()),
            };

            // make proposal
            let mut proposal = Proposal {
                r#type: SignedMsgType::Proposal.into(),
                height: rs.height,
                round: rs.round,
                pol_round: rs.valid_round,
                block_id: Some(block_id),
                timestamp: Some(timestamp::now()),
                signature: vec![],
            };

            let priv_validator = self.clone().priv_validator.clone();
            let chain_id = self.state.clone().get_chain_id();

            if let Ok(_) = priv_validator
                .clone()
                .sign_proposal(chain_id, &mut proposal)
            {
                let msg_info = MessageInfo {
                    msg: Arc::new(ConsensusMessageType::ProposalMessage(ProposalMessage {
                        proposal: Some(proposal),
                    })),
                    peer_id: internal_peerid(),
                };

                _ = self.msg_chan_sender.try_send(msg_info);
            } else {
                log::error!("sign proposal failed");
                return;
            }
        } else {
            log::error!("invalid block or block parts")
        }
    }

    // auxiliary functions

    fn create_proposal_block(self: Arc<Self>) -> (Option<Block>, Option<PartSet>) {
        todo!()
    }

    fn is_proposer(self: Arc<Self>) -> bool {
        match self.clone().priv_validator.clone().get_address() {
            Some(priv_validator_addr) => {
                if let Ok(mut rs_guard) = self.rs.clone().lock() {
                    let vs = rs_guard.validators.as_mut();
                    if let Some(proposer) = vs.and_then(|vs| vs.get_proposer()) {
                        return priv_validator_addr.eq(&proposer.address);
                    } else {
                        return false;
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /**
    Implements create vote and sign vote.
    Nil priv validator returns empty vote.
    Panics if any errors.
     */
    fn create_signed_vote(
        self: Arc<Self>,
        rs: RoundState,
        vote_type: SignedMsgType,
        block_id: BlockId,
    ) -> Result<Vote, ConsensusStateError> {
        match self.clone().priv_validator.get_address() {
            Some(validator_address) => {
                match rs
                    .validators
                    .and_then(|v| v.get_by_address(validator_address))
                {
                    Some(iv) => {
                        let (validator_index, _) = iv;

                        // create unsigned vote
                        let mut vote = Vote {
                            r#type: vote_type.into(),
                            height: rs.height,
                            round: rs.round,
                            block_id: Some(block_id),
                            timestamp: Some(timestamp::now()),
                            validator_address: validator_address.as_bytes().to_vec(),
                            validator_index: validator_index.try_into().unwrap(),
                            signature: vec![], // set nil for now, will be signed below
                        };

                        let chain_id = self.state.clone().get_chain_id();

                        // signing vote
                        if let Ok(_) = self.clone().priv_validator.sign_vote(chain_id, &mut vote) {
                            return Ok(vote);
                        } else {
                            return Err(CreateSignedVoteError("sign vote failed".to_owned()));
                        }
                    }
                    None => Err(CreateSignedVoteError(
                        "cannot find validator index".to_owned(),
                    )),
                }
            }
            None => Err(CreateSignedVoteError("nil priv validator".to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ops::Add, sync::Arc, thread, time::Duration};

    use bytes::Bytes;
    use kai_types::{
        block::BlockId,
        block_operations::MockBlockOperations,
        consensus::{
            executor::MockBlockExecutor,
            height_vote_set::{HeightVoteSet, RoundVoteSet},
            state::MockLatestBlockState,
        },
        crypto::Proof,
        evidence::MockEvidencePool,
        part_set::{Part, PartSet, PartSetHeader},
        priv_validator::MockPrivValidator,
        proposal::{proposal_sign_bytes, Proposal},
        round::RoundStep,
        types::SignedMsgType,
        validator_set::{Validator, ValidatorSet},
        vote::Vote,
        vote_set::VoteSet,
    };
    use tokio::runtime::Runtime;

    use crate::{
        state::{ConsensusState, ConsensusStateImpl},
        types::{
            config::ConsensusConfig,
            messages::{
                BlockPartMessage, ConsensusMessageType, MessageInfo, ProposalMessage, VoteMessage,
            },
            peer::internal_peerid,
        },
    };

    use ethereum_types::Address;

    /**
     * ------------------------
     * Constants
     * ------------------------
     */

    pub const ADDR_1: Address =
        ethereum_types::H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    pub const ADDR_2: Address =
        ethereum_types::H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    pub const ADDR_3: Address =
        ethereum_types::H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]);

    pub const VAL_1: Validator = Validator {
        address: ADDR_1,
        voting_power: 1,
        proposer_priority: 1,
    };
    pub const VAL_2: Validator = Validator {
        address: ADDR_2,
        voting_power: 2,
        proposer_priority: 2,
    };
    pub const VAL_3: Validator = Validator {
        address: ADDR_3,
        voting_power: 3,
        proposer_priority: 3,
    };

    /**
     * ------------------------
     * Internal functions tests
     * ------------------------
     */

    #[test]
    fn internal_send() {
        let m_latest_block_state = MockLatestBlockState::new();
        let m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        let cs = ConsensusStateImpl::new(
            ConsensusConfig::new_default(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );

        let msg = Arc::new(ConsensusMessageType::ProposalMessage(ProposalMessage {
            proposal: None,
        }));

        let arc_cs = Arc::new(cs);
        let arc_cs_1 = arc_cs.clone();

        let rc = thread::spawn(move || {
            if let Ok(mut rx_guard) = arc_cs.clone().msg_chan_receiver.lock() {
                let rev_msg = rx_guard.blocking_recv();
                assert!(rev_msg.is_some());

                let msg_info = rev_msg.unwrap();

                assert!(
                    msg_info.clone().peer_id == internal_peerid()
                        && matches!(
                            msg_info.clone().msg.clone().as_ref(),
                            ConsensusMessageType::ProposalMessage(_)
                        )
                );
            } else {
                panic!("should lock");
            }
        });

        Runtime::new().unwrap().block_on(async move {
            let _ = arc_cs_1
                .clone()
                .send(MessageInfo {
                    msg: msg.clone(),
                    peer_id: internal_peerid(),
                })
                .await;
        });

        rc.join().unwrap();
    }

    #[test]
    fn process_msg_chan() {
        let m_latest_block_state = MockLatestBlockState::new();
        let m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        let cs = ConsensusStateImpl::new(
            ConsensusConfig::new_default(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );

        // here we send invalid proposal
        let msg = Arc::new(ConsensusMessageType::ProposalMessage(ProposalMessage {
            proposal: None,
        }));

        let arc_cs = Arc::new(cs);
        let arc_cs_1 = arc_cs.clone();

        Runtime::new().unwrap().block_on(async move {
            // start processing messages
            let msg_thr = tokio::spawn(async move {
                arc_cs.process_msg_chan();
            });

            _ = arc_cs_1
                .send(MessageInfo {
                    msg: msg,
                    peer_id: internal_peerid(),
                })
                .await;

            // wait for a while for processing message
            thread::sleep(Duration::from_millis(200));

            // stop the consensus state
            // closes the channel
            // msg process will stop
            _ = arc_cs_1.stop();

            let join_rs = msg_thr.await;
            assert!(join_rs.is_ok());
        });
    }

    use kai_lib::{
        crypto::{
            crypto::{keccak256, pub_to_address},
            signature::sign,
        },
        secp256k1::{self, SECP256K1},
    };

    #[test]
    fn set_proposal() {
        let mut m_latest_block_state = MockLatestBlockState::new();
        let m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        let m_chain_id: String = "".to_string();

        m_latest_block_state
            .expect_get_chain_id()
            .return_const(m_chain_id.clone());

        let cs = ConsensusStateImpl::new(
            ConsensusConfig::new_default(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );

        // create unsigned proposal
        let mut m_proposal = Proposal {
            r#type: SignedMsgType::Proposal.into(),
            height: 1,
            round: 1,
            pol_round: 0,
            block_id: Some(BlockId {
                hash: keccak256(Bytes::new()).to_vec(),
                part_set_header: Some(PartSetHeader {
                    total: 10,
                    hash: keccak256(Bytes::new()).to_vec(),
                }),
            }),
            timestamp: None,
            signature: vec![],
        };

        // sign the proposal
        let psb = proposal_sign_bytes(m_chain_id.clone(), m_proposal.clone()).unwrap();
        let signer_secret_key = secp256k1::SecretKey::new(&mut secp256k1::rand::thread_rng());
        let signer_public_key =
            secp256k1::PublicKey::from_secret_key(SECP256K1, &signer_secret_key);

        let signature = sign(keccak256(psb), signer_secret_key).unwrap();
        m_proposal.signature = signature.to_vec();

        // create proposer validator
        let proposer: Validator = Validator {
            address: pub_to_address(signer_public_key),
            voting_power: 4,
            proposer_priority: 4,
        };

        let val_set: ValidatorSet = ValidatorSet {
            validators: vec![VAL_1, VAL_2, VAL_3, proposer],
            proposer: None,
            total_voting_power: 10,
        };

        let msg = Arc::new(ConsensusMessageType::ProposalMessage(ProposalMessage {
            proposal: Some(m_proposal.clone()),
        }));

        let arc_cs = Arc::new(cs);
        let arc_cs_1 = arc_cs.clone();
        let arc_cs_2 = arc_cs.clone();

        // arrange round state
        let mut rs_guard = arc_cs
            .rs
            .lock()
            .expect("could not lock round state for arrangement");
        rs_guard.validators = Some(val_set);
        drop(rs_guard);

        Runtime::new().unwrap().block_on(async move {
            // start processing messages
            let msg_thr = tokio::spawn(async move {
                arc_cs.process_msg_chan();
            });

            // send proposal message
            _ = arc_cs_1
                .send(MessageInfo {
                    msg: msg,
                    peer_id: internal_peerid(),
                })
                .await;

            // wait for a while for processing message
            thread::sleep(Duration::from_millis(200));

            // stop the consensus state
            // closes the channel
            // msg process will stop
            _ = arc_cs_1.stop();

            // make sure this processing message thread is stopped
            let join_rs = msg_thr.await;
            assert!(join_rs.is_ok());

            let rs = arc_cs_2.clone().get_rs().unwrap();
            // assert proposal
            assert!(
                rs.proposal.is_some()
                    && rs.proposal_block.is_none()
                    && rs.proposal_block_parts.is_some()
            );
        });
    }

    #[test]
    fn add_proposal_block_part() {
        let mut m_latest_block_state = MockLatestBlockState::new();
        let m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        let m_chain_id: String = "".to_string();

        m_latest_block_state
            .expect_get_chain_id()
            .return_const(m_chain_id.clone());

        let cs = ConsensusStateImpl::new(
            ConsensusConfig::new_default(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );

        let total_parts = 10;

        let val_set: ValidatorSet = ValidatorSet {
            validators: vec![VAL_1, VAL_2, VAL_3],
            proposer: None,
            total_voting_power: 6,
        };

        let arc_cs = Arc::new(cs);
        let arc_cs_1 = arc_cs.clone();
        let arc_cs_2 = arc_cs.clone();

        // arrange round state
        let mut rs_guard = arc_cs
            .rs
            .lock()
            .expect("could not lock round state for arrangement");
        rs_guard.validators = Some(val_set);
        rs_guard.proposal_block_parts = Some(PartSet::new_part_set_from_header(PartSetHeader {
            total: total_parts,
            hash: keccak256(Bytes::new()).to_vec(),
        }));
        drop(rs_guard);

        Runtime::new().unwrap().block_on(async move {
            // start processing messages
            let msg_thr = tokio::spawn(async move {
                arc_cs.process_msg_chan();
            });

            // send block part messages
            let part_0_index = 0; // first
            let part_1_index = 1; // second

            let m_block_part_0 = BlockPartMessage {
                height: 1,
                round: 1,
                part: Some(Part {
                    index: part_0_index,
                    bytes: vec![],
                    proof: Some(Proof {
                        total: total_parts as u64,
                        index: part_0_index as u64,
                        leaf_hash: vec![],
                        aunts: vec![vec![]],
                    }),
                }),
            };
            let msg = Arc::new(ConsensusMessageType::BlockPartMessage(m_block_part_0));
            _ = arc_cs_1
                .send(MessageInfo {
                    msg: msg,
                    peer_id: internal_peerid(),
                })
                .await;

            let m_block_part_1 = BlockPartMessage {
                height: 1,
                round: 1,
                part: Some(Part {
                    index: part_1_index,
                    bytes: vec![],
                    proof: Some(Proof {
                        total: total_parts as u64,
                        index: part_1_index as u64,
                        leaf_hash: vec![],
                        aunts: vec![vec![]],
                    }),
                }),
            };

            let msg = Arc::new(ConsensusMessageType::BlockPartMessage(m_block_part_1));
            _ = arc_cs_1
                .send(MessageInfo {
                    msg: msg,
                    peer_id: internal_peerid(),
                })
                .await;

            // wait for a while for processing message
            thread::sleep(Duration::from_millis(200));

            // stop the consensus state
            // closes the channel
            // msg process will stop
            _ = arc_cs_1.stop();

            // make sure this processing message thread is stopped
            let join_rs = msg_thr.await;
            assert!(join_rs.is_ok());

            let rs = arc_cs_2.clone().get_rs().unwrap();
            // assert proposal block part
            let pbp = rs.proposal_block_parts.unwrap();
            assert_eq!(pbp.count, 2); // received 1 part
            assert_eq!(pbp.total, total_parts);
            assert!(pbp.parts.get(part_0_index as usize).is_some());
            assert!(pbp.parts.get(part_1_index as usize).is_some());
            assert!(pbp
                .parts_bit_array
                .get_index(part_0_index as usize)
                .is_ok_and(|v| *v == true));
            assert!(pbp
                .parts_bit_array
                .get_index(part_1_index as usize)
                .is_ok_and(|v| *v == true));
        });
    }

    #[test]
    fn add_vote() {
        let mut m_latest_block_state = MockLatestBlockState::new();
        let m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        let m_chain_id: String = "".to_string();

        m_latest_block_state
            .expect_get_chain_id()
            .return_const(m_chain_id.clone());

        let cs = ConsensusStateImpl::new(
            ConsensusConfig::new_default(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );

        let total_parts = 10;

        let val_set: ValidatorSet = ValidatorSet {
            validators: vec![VAL_1, VAL_2, VAL_3],
            proposer: None,
            total_voting_power: 6,
        };

        let arc_cs = Arc::new(cs);
        let arc_cs_1 = arc_cs.clone();
        let arc_cs_2 = arc_cs.clone();

        // arrange round state
        let mut rs_guard = arc_cs
            .rs
            .lock()
            .expect("could not lock round state for arrangement");
        // TODO: set up round state
        rs_guard.validators = Some(val_set.clone());
        rs_guard.votes = Some(HeightVoteSet {
            chain_id: m_chain_id,
            height: 1,
            round: 1,
            validator_set: Some(val_set),
            round_vote_sets: HashMap::new(),
            peer_catchup_rounds: HashMap::new(),
        });
        drop(rs_guard);

        Runtime::new().unwrap().block_on(async move {
            // start processing messages
            let msg_thr = tokio::spawn(async move {
                arc_cs.process_msg_chan();
            });

            let vote_msg = VoteMessage {
                vote: Some(Vote {
                    r#type: SignedMsgType::Precommit.into(),
                    height: 1,
                    round: 1,
                    block_id: Some(BlockId::new_zero_block_id()),
                    timestamp: None,
                    validator_address: VAL_1.address.0.to_vec(),
                    validator_index: 1,
                    signature: vec![],
                }),
            };
            let msg = Arc::new(ConsensusMessageType::VoteMessage(vote_msg));
            _ = arc_cs_1
                .send(MessageInfo {
                    msg: msg,
                    peer_id: internal_peerid(),
                })
                .await;

            // wait for a while for processing message
            thread::sleep(Duration::from_millis(200));

            // stop the consensus state
            // closes the channel
            // msg process will stop
            _ = arc_cs_1.stop();

            // make sure this processing message thread is stopped
            let join_rs = msg_thr.await;
            assert!(join_rs.is_ok());

            let rs = arc_cs_2.clone().get_rs().unwrap();

            // TODO: assertions.
        });
    }

    #[test]
    fn is_proposer() {
        let m_latest_block_state = MockLatestBlockState::new();
        let mut m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        m_priv_validator.expect_get_address().return_const(ADDR_2);

        let val_set: ValidatorSet = ValidatorSet {
            validators: vec![VAL_1, VAL_2, VAL_3],
            proposer: None,
            total_voting_power: 6,
        };

        let cs = ConsensusStateImpl::new(
            ConsensusConfig::new_default(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );

        let acs = Arc::new(cs);

        // arrange round state
        if let Ok(mut rs_guard) = acs.clone().rs.clone().lock() {
            rs_guard.validators = Some(val_set);
            drop(rs_guard);
        } else {
            panic!("could not lock round state for arrangement")
        }

        let is_proposer = acs.clone().is_proposer();
        assert!(!is_proposer);

        if let Ok(rs_guard) = acs.clone().rs.clone().lock() {
            let validator_set = rs_guard.clone().validators;
            assert!(
                validator_set.is_some_and(|vs| vs.proposer.is_some_and(|p| p.address.eq(&ADDR_3)))
            );
        } else {
            panic!("could not lock round state for arrangement")
        }
    }

    /**
     * ------------------------
     * State transitions tests
     * ------------------------
     */

    #[test]
    fn propose_timeout_send_prevote_for_nil() {
        let mut m_latest_block_state = MockLatestBlockState::new();
        let mut m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        m_latest_block_state
            .expect_get_chain_id()
            .return_const("".to_string());

        let mut signature: Vec<u8> = vec![4, 5, 6];

        m_priv_validator.expect_get_address().return_const(ADDR_2);
        m_priv_validator
            .expect_sign_vote()
            .returning(move |_, vote| {
                vote.signature.append(&mut signature);
                Ok(())
            });

        let mut config = ConsensusConfig::new_default();
        config.timeout_propose = Duration::from_millis(100);

        let cs = ConsensusStateImpl::new(
            config.clone(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );
        let arc_cs = Arc::new(cs);
        let arc_cs_2 = arc_cs.clone();

        let val_set: ValidatorSet = ValidatorSet {
            validators: vec![VAL_1, VAL_2, VAL_3],
            proposer: None,
            total_voting_power: 6,
        };

        // arrange round state
        let mut rs_guard = arc_cs
            .rs
            .lock()
            .expect("could not lock round state for arrangement");
        rs_guard.height = 1;
        rs_guard.round = 1;
        rs_guard.validators = Some(val_set);
        drop(rs_guard);

        let rc = thread::spawn(move || {
            if let Ok(mut rx_guard) = arc_cs.clone().msg_chan_receiver.lock() {
                let rev_msg = rx_guard.blocking_recv();
                assert!(rev_msg.is_some());

                let msg_info = rev_msg.unwrap();
                assert!(
                    msg_info.clone().peer_id == internal_peerid()
                        && match msg_info.clone().msg.clone().as_ref() {
                            ConsensusMessageType::VoteMessage(vm) => vm.vote.is_some_and(|v| v
                                .block_id
                                .is_some_and(
                                    |bid| bid.hash.len() == 0 && bid.part_set_header == None
                                )),
                            _ => false,
                        }
                );
            } else {
                panic!("should lock");
            }
        });

        let timeout_propose = config.timeout_propose;
        Runtime::new().unwrap().block_on(async move {
            if let Ok(_) = arc_cs_2.clone().start() {
            } else {
                panic!("should not failed to start");
            }

            // wait for propose timeout happens
            thread::sleep(timeout_propose.add(timeout_propose));

            // assertions
            if let Some(rs) = arc_cs_2.clone().get_rs() {
                assert_eq!(rs.step, RoundStep::Prevote);
            } else {
                panic!("should get round state")
            }
        });

        rc.join().unwrap();
    }

    #[test]
    fn prevote_timeout_send_precommit_for_nil() {
        let mut m_latest_block_state = MockLatestBlockState::new();
        let mut m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        m_latest_block_state
            .expect_get_chain_id()
            .return_const("".to_string());

        let mut signature: Vec<u8> = vec![4, 5, 6];

        m_priv_validator.expect_get_address().return_const(ADDR_2);
        m_priv_validator
            .expect_sign_vote()
            .returning(move |_, vote| {
                vote.signature.append(&mut signature);
                Ok(())
            });

        let mut config = ConsensusConfig::new_default();
        config.timeout_propose = Duration::from_millis(100);
        config.timeout_prevote = Duration::from_millis(100);

        let cs = ConsensusStateImpl::new(
            config.clone(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );
        let arc_cs = Arc::new(cs);
        let arc_cs_1 = arc_cs.clone();

        let val_set: ValidatorSet = ValidatorSet {
            validators: vec![VAL_1, VAL_2, VAL_3],
            proposer: None,
            total_voting_power: 6,
        };

        // arrange round state
        if let Ok(mut rs_guard) = arc_cs.clone().rs.lock() {
            rs_guard.height = 1;
            rs_guard.round = 1;
            rs_guard.step = RoundStep::Propose;
            rs_guard.validators = Some(val_set.clone());
            let rs = rs_guard.clone();
            let mut hvs = HeightVoteSet {
                chain_id: "".to_owned(),
                height: 1,
                round: 1,
                validator_set: Some(val_set.clone()),
                round_vote_sets: HashMap::new(),
                peer_catchup_rounds: HashMap::new(),
            };
            hvs.round_vote_sets.insert(
                rs.round,
                RoundVoteSet {
                    prevotes: Some(VoteSet {
                        maj23: Some(BlockId::new_zero_block_id()),
                    }),
                    precommits: Some(VoteSet {
                        maj23: Some(BlockId::new_zero_block_id()),
                    }),
                },
            );

            rs_guard.votes = Some(hvs);
            drop(rs_guard);
        } else {
            panic!("could not lock round state for arrangement")
        }

        let timeout_propose = config.timeout_propose;
        let timeout_prevote = config.timeout_prevote;

        // start this task first
        thread::spawn(move || {
            let cs = arc_cs_1.clone();
            cs.process_msg_chan();
        });

        // then starts this task and wait until complete
        Runtime::new().unwrap().block_on(async move {
            let cs = arc_cs.clone();
            // start consensus state at new round
            cs.clone().start().expect("should start successfully");

            // wait for propose timeout happens
            thread::sleep(timeout_propose.add(timeout_propose));

            // wait for prevote timeout happens
            thread::sleep(timeout_prevote.add(timeout_prevote));

            // assertions
            let rs = cs.clone().get_rs().expect("should get round state");
            assert_eq!(rs.step, RoundStep::Precommit);
        });
    }

    #[test]
    fn precommit_timeout_start_new_round() {
        let mut m_latest_block_state = MockLatestBlockState::new();
        let mut m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        m_latest_block_state
            .expect_get_chain_id()
            .return_const("".to_string());

        let mut signature: Vec<u8> = vec![4, 5, 6];

        m_priv_validator.expect_get_address().return_const(ADDR_2);
        m_priv_validator
            .expect_sign_vote()
            .returning(move |_, vote| {
                vote.signature.append(&mut signature);
                Ok(())
            });

        let mut config = ConsensusConfig::new_default();
        config.timeout_propose = Duration::from_millis(100);
        config.timeout_prevote = Duration::from_millis(100);
        config.timeout_precommit = Duration::from_millis(100);

        let cs = ConsensusStateImpl::new(
            config.clone(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );
        let arc_cs = Arc::new(cs);
        let arc_cs_1 = arc_cs.clone();

        let val_set: ValidatorSet = ValidatorSet {
            validators: vec![VAL_1, VAL_2, VAL_3],
            proposer: None,
            total_voting_power: 6,
        };

        // arrange round state
        let binding = arc_cs_1.clone();
        let mut rs_guard = binding.rs.lock().expect("should lock ok");
        rs_guard.height = 1;
        rs_guard.round = 1;
        rs_guard.step = RoundStep::Propose;
        rs_guard.validators = Some(val_set.clone());
        let rs = rs_guard.clone();
        let mut hvs = HeightVoteSet {
            chain_id: "".to_owned(),
            height: 1,
            round: 1,
            validator_set: Some(val_set.clone()),
            round_vote_sets: HashMap::new(),
            peer_catchup_rounds: HashMap::new(),
        };
        hvs.round_vote_sets.insert(
            rs.round,
            RoundVoteSet {
                prevotes: Some(VoteSet {
                    maj23: Some(BlockId::new_zero_block_id()),
                }),
                precommits: Some(VoteSet {
                    maj23: Some(BlockId::new_zero_block_id()),
                }),
            },
        );

        rs_guard.votes = Some(hvs);
        drop(rs_guard);

        let timeout_propose = config.timeout_propose;
        let timeout_prevote = config.timeout_prevote;
        let timeout_precommit = config.timeout_precommit;

        // start this task first
        thread::spawn(move || {
            let cs = arc_cs_1.clone();
            cs.process_msg_chan();
        });

        // then starts this task and wait until complete
        Runtime::new().unwrap().block_on(async move {
            let cs = arc_cs.clone();
            // start consensus state at new round
            cs.clone().start().expect("should start successfully");
            let last_rs = cs.clone().get_rs().expect("should get round state");

            // wait for propose timeout happens
            thread::sleep(timeout_propose.add(timeout_propose));

            // wait for prevote timeout happens
            thread::sleep(timeout_prevote.add(timeout_prevote));

            // wait for precommit timeout happens
            thread::sleep(timeout_precommit.add(timeout_precommit));

            // assertions
            let rs = cs.clone().get_rs().expect("should get round state");
            assert_eq!(rs.step, RoundStep::Propose);
            assert_eq!(rs.height, last_rs.height); // assert height
            assert_eq!(rs.round, last_rs.round + 1); // assert new round
        });
    }
}
