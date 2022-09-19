use crate::types::{
    config::ConsensusConfig,
    error::ConsensusStateError::{self, CreateSignedVoteError},
    messages::{
        BlockPartMessage, ConsensusMessage, ConsensusMessageType, MessageInfo, ProposalMessage,
        VoteMessage,
    },
    peer::internal_peerid,
    round_state::{RoundState, RULE_4},
};

use kai_types::{
    block::{new_zero_block_id, Block, BlockId},
    block_operations::BlockOperations,
    consensus::{executor::BlockExecutor, state::LatestBlockState},
    evidence::EvidencePool,
    part_set::PartSet,
    priv_validator::PrivValidator,
    proposal::Proposal,
    round::RoundStep,
    timestamp,
    types::SignedMsgType,
    vote::Vote,
};
use log::debug;
use mockall::automock;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    fmt::Debug,
    ops::{Add, Mul},
    sync::{Arc, Mutex},
    thread,
};
use tokio::sync::mpsc::{error::TrySendError, Receiver, Sender};

const MSG_QUEUE_SIZE: usize = 1000;

#[automock]
pub trait ConsensusState: Debug + Send + Sync + 'static {
    fn get_config(&self) -> Arc<ConsensusConfig>;
    fn get_state(&self) -> Arc<Box<dyn LatestBlockState>>;
    fn get_block_operations(&self) -> Arc<Box<dyn BlockOperations>>;
    fn get_block_exec(&self) -> Arc<Box<dyn BlockExecutor>>;
    fn get_evidence_pool(&self) -> Arc<Box<dyn EvidencePool>>;
    fn get_rs(&self) -> Option<RoundState>;
    fn send_peer_msg_chan(&self, msg_info: MessageInfo);
    fn send_internal_msg_chan(&self, msg_info: MessageInfo);
    fn send(self: Arc<Self>, msg_info: MessageInfo) -> Result<(), TrySendError<MessageInfo>>;

    fn update_to_state(&self, state: Arc<Box<dyn LatestBlockState>>);
    fn start(self: Arc<Self>) -> Result<(), Box<ConsensusStateError>>;
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

#[derive(Debug)]
pub struct ConsensusMsgChan<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

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

    fn send_peer_msg_chan(&self, msg_info: MessageInfo) {
        todo!()
    }

    fn send_internal_msg_chan(&self, msg_info: MessageInfo) {
        todo!()
    }

    fn send(self: Arc<Self>, msg_info: MessageInfo) -> Result<(), TrySendError<MessageInfo>> {
        self.clone().msg_chan_sender.try_send(msg_info)
    }

    fn start(self: Arc<Self>) -> Result<(), Box<ConsensusStateError>> {
        // TODO:

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
        let (msg_chan_sender, mut msg_chan_receiver) = tokio::sync::mpsc::channel(MSG_QUEUE_SIZE);

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

    async fn process_msg_chan(self: Arc<Self>, mut msg_chan_receiver: Receiver<MessageInfo>) {
        while let Some(msg_info) = msg_chan_receiver.recv().await {
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
                    if let Err(e) = self.clone().try_add_vote(_msg.clone()) {
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
    }

    fn check_upon_rules(self: Arc<Self>, msg: Arc<ConsensusMessageType>) {
        let msg = msg.clone();
        match msg.as_ref() {
            ConsensusMessageType::ProposalMessage(_msg) => {
                log::debug!(
                    "checking upon rules for proposal: proposal={:?}",
                    _msg.clone()
                );
                todo!()
            }
            ConsensusMessageType::VoteMessage(_msg) => {
                log::debug!("set vote: vote={:?}", _msg.clone());

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
                                // checking rule #4
                                if let Some(_) = rs
                                    .votes
                                    .expect("vote should not nil")
                                    .prevotes(rs.round)
                                    .expect("prevotes should not nil")
                                    .two_thirds_majority()
                                {
                                    // if rule has not been triggered
                                    if !rs_guard.triggered_rules.contains(&RULE_4) {
                                        self.clone().schedule_timeout(rs.height, rs.round, rs.step);
                                        // mark rule has triggered
                                        rs_guard.triggered_rules.insert(RULE_4);
                                    }
                                }

                                // checking rule #6
                            }
                            drop(rs_guard);
                        } else {
                            log::trace!("lock round state failed")
                        }
                    }
                    SignedMsgType::Precommit => {
                        todo!()
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
        todo!()

        // check for existing proposal...

        // TODO: check upons rule
    }

    /**
    This function adds proposal block part and it checks:
    * full proposal block, then it makes state transition to PREVOTE step
    */
    fn add_proposal_block_part(&self, msg: BlockPartMessage) -> Result<(), ConsensusStateError> {
        todo!()

        // check for proposal

        // add block part

        // check full proposal block
        //    => switch to prevote
        //    => if valid block => send prevote

        // TODO: check upons rule
    }

    fn try_add_vote(self: Arc<Self>, msg: VoteMessage) -> Result<(), ConsensusStateError> {
        todo!()

        // route votes
        // - listen for 2/3 votes

        // TODO: check upons rule
    }

    fn schedule_timeout(self: Arc<Self>, height: u64, round: u32, step: RoundStep) {
        match step {
            RoundStep::Propose => {
                let timeout_propose = self.clone().get_config().timeout_propose.clone();
                let timeout_propose_delta = self.clone().get_config().timeout_propose_delta.clone();
                thread::spawn(move || {
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
                thread::spawn(move || {
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
                thread::spawn(move || {
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
                    new_zero_block_id(), // nil block
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
                    new_zero_block_id(), // nil block
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
                            validator_address: validator_address.to_vec(),
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
    use std::{ops::Add, sync::Arc, thread, time::Duration};

    use kai_types::{
        block_operations::MockBlockOperations,
        common::address::Address,
        consensus::{executor::MockBlockExecutor, state::MockLatestBlockState},
        evidence::MockEvidencePool,
        priv_validator::MockPrivValidator,
        round::RoundStep,
        types::SignedMsgType,
        validator_set::{Validator, ValidatorSet},
    };
    use tokio::runtime::Runtime;

    use crate::types::{
        config::ConsensusConfig,
        messages::{ConsensusMessageType, MessageInfo, ProposalMessage},
        peer::internal_peerid,
    };

    use super::{ConsensusState, ConsensusStateImpl};

    #[test]
    fn send() {
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

        let rc = thread::spawn(move || {
            if let Ok(mut rx_guard) = cs.msg_chan_receiver.lock() {
                let rev_msg = rx_guard.blocking_recv();
                assert!(rev_msg.is_some());

                let msg_info = rev_msg.unwrap();

                assert!(
                    msg_info.clone().peer_id == internal_peerid()
                        && matches!(
                            msg_info.clone().msg.clone().as_ref(),
                            ConsensusMessageType::ProposalMessage(p)
                        )
                );
            } else {
                panic!("should lock");
            }
        });

        Runtime::new().unwrap().block_on(async move {
            let _ = cs
                .msg_chan_sender
                .send(MessageInfo {
                    msg: msg.clone(),
                    peer_id: internal_peerid(),
                })
                .await;
        });

        rc.join().unwrap();
    }

    pub const ADDR_1: Address = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    pub const ADDR_2: Address = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
    pub const ADDR_3: Address = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];

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

    #[test]
    fn is_proposer() {
        let m_latest_block_state = MockLatestBlockState::new();
        let mut m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        // set ADDR_2 for our priv validator
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
        if let Ok(mut rs_guard) = arc_cs.clone().rs.lock() {
            rs_guard.height = 1;
            rs_guard.round = 1;
            rs_guard.validators = Some(val_set);
            drop(rs_guard);
        } else {
            panic!("could not lock round state for arrangement")
        }

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
        let arc_cs_2 = arc_cs.clone();

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
            rs_guard.validators = Some(val_set);
            drop(rs_guard);
        } else {
            panic!("could not lock round state for arrangement")
        }

        // thread listens for msg and do assertions.
        let rc = thread::spawn(move || {
            if let Ok(mut rx_guard) = arc_cs.clone().msg_chan_receiver.lock() {
                let mut total_received_msg = 0;
                while let Some(rev_msg) = rx_guard.blocking_recv() {
                    total_received_msg += 1;

                    match total_received_msg {
                        1 => {
                            let msg_info = rev_msg;
                            assert!(
                                msg_info.clone().peer_id == internal_peerid()
                                    && match msg_info.clone().msg.clone().as_ref() {
                                        ConsensusMessageType::VoteMessage(vm) =>
                                            vm.vote.is_some_and(|v| v
                                                .block_id
                                                .is_some_and(|bid| bid.hash.len() == 0
                                                    && bid.part_set_header == None)
                                                && matches!(
                                                    v.r#type.into(),
                                                    SignedMsgType::Prevote
                                                )),
                                        _ => false,
                                    }
                            );
                        }
                        2 => {
                            let msg_info = rev_msg;
                            assert!(
                                msg_info.clone().peer_id == internal_peerid()
                                    && match msg_info.clone().msg.clone().as_ref() {
                                        ConsensusMessageType::VoteMessage(vm) =>
                                            vm.vote.is_some_and(|v| v
                                                .block_id
                                                .is_some_and(|bid| bid.hash.len() == 0
                                                    && bid.part_set_header == None)
                                                && matches!(
                                                    v.r#type.into(),
                                                    SignedMsgType::Precommit
                                                )),
                                        _ => false,
                                    }
                            );
                        }
                        _ => {}
                    }
                }
            } else {
                panic!("should lock");
            }
        });

        let timeout_propose = config.timeout_propose;
        let timeout_prevote = config.timeout_prevote;
        Runtime::new().unwrap().block_on(async move {
            // start consensus state
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

            // wait for prevote timeout happens
            thread::sleep(timeout_prevote.add(timeout_prevote));

            // assertions
            if let Some(rs) = arc_cs_2.clone().get_rs() {
                assert_eq!(rs.step, RoundStep::Precommit);
            } else {
                panic!("should get round state")
            }
        });

        rc.join().unwrap();
    }
}
