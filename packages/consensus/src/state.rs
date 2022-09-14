use crate::types::{
    config::ConsensusConfig,
    error::{ConsensusReactorError, ConsensusStateError},
    messages::{
        BlockPartMessage, ConsensusMessage, ConsensusMessageType, MessageInfo, ProposalMessage,
        VoteMessage,
    },
    peer::internal_peerid,
    round_state::RoundState,
};
use kai_types::{
    block::{Block, BlockId},
    block_operations::BlockOperations,
    consensus::{executor::BlockExecutor, state::LatestBlockState},
    evidence::EvidencePool,
    part_set::PartSet,
    priv_validator::PrivValidator,
    proposal::Proposal,
    round::RoundStep,
    timestamp,
    types::SignedMsgType,
};
use mockall::automock;
use std::{
    fmt::Debug,
    ops::{Add, Mul},
    sync::{Arc, Mutex, MutexGuard},
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
    pub peer_msg_chan: (
        Sender<Box<dyn ConsensusMessage>>,
        Receiver<Box<dyn ConsensusMessage>>,
    ),
    pub internal_msg_chan: (
        Sender<Box<dyn ConsensusMessage>>,
        Receiver<Box<dyn ConsensusMessage>>,
    ),
    pub msg_chan_sender: Sender<MessageInfo>,
    pub msg_chan_receiver: Receiver<MessageInfo>,
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
        let (msg_chan_sender, msg_chan_receiver) = tokio::sync::mpsc::channel(MSG_QUEUE_SIZE);

        Self {
            config: Arc::new(config),
            state: state.clone(),
            priv_validator: priv_validator,
            block_operations: block_operations,
            block_exec: block_exec,
            evidence_pool: evidence_pool,
            rs: Arc::new(Mutex::new(RoundState::new_default())),
            peer_msg_chan: tokio::sync::mpsc::channel(MSG_QUEUE_SIZE),
            internal_msg_chan: tokio::sync::mpsc::channel(MSG_QUEUE_SIZE),
            msg_chan_sender: msg_chan_sender,
            msg_chan_receiver: msg_chan_receiver,
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
        todo!()
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

    fn on_timeout_propose(self: Arc<Self>, height: u64, round: u32) {
        if let Ok(mut rs_guard) = self.rs.clone().lock() {
            if height == rs_guard.height
                && round == rs_guard.round
                && rs_guard.step == RoundStep::Propose
            {
                // TODO:
                // broadcast <PREVOTE, h_p, round_p, nil>
                // self.out_msg_chan.tx.try_send(message)
            }
            rs_guard.step = RoundStep::Precommit;
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
                // TODO:
                // broadcast <PRECOMMIT, h_p, round_p, nil>
                // self.out_msg_chan.tx.try_send(message)
            }
            rs_guard.step = RoundStep::Precommit;
            drop(rs_guard);
        } else {
            log::trace!("lock round state failed")
        }
    }

    fn on_timeout_precommit(self: Arc<Self>, height: u64, round: u32) {
        if let Ok(rs_guard) = self.rs.clone().lock() {
            if height == rs_guard.height && round == rs_guard.round {
                // TODO: self.start_round(rs_guard.round + 1);
            }
            drop(rs_guard);
        } else {
            log::trace!("lock round state failed")
        }
    }

    fn start_new_round(self: Arc<Self>, round: u32) {
        if let Ok(mut rs_guard) = self.rs.clone().lock() {
            rs_guard.round = round;
            rs_guard.step = RoundStep::Propose;
            let rs = rs_guard.clone();
            drop(rs_guard);

            if self.clone().is_proposer() {
                self.clone().decide_proposal(rs);
            } else {
                let timeout_propose = self.clone().get_config().timeout_propose.clone();
                let timeout_propose_delta = self.clone().get_config().timeout_propose_delta.clone();
                thread::spawn(move || {
                    let sleep_duration = if round > 1 {
                        timeout_propose.add(timeout_propose_delta.mul(round - 1))
                    } else {
                        timeout_propose
                    };
                    thread::sleep(sleep_duration);
                    self.clone().on_timeout_propose(rs.height, rs.round);
                });
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
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};

    use kai_types::{
        block_operations::MockBlockOperations,
        consensus::{executor::MockBlockExecutor, state::MockLatestBlockState},
        evidence::MockEvidencePool,
        priv_validator::MockPrivValidator,
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

        let mut rx = cs.msg_chan_receiver;

        let msg = Arc::new(ConsensusMessageType::ProposalMessage(ProposalMessage {
            proposal: None,
        }));

        let rc = thread::spawn(move || {
            let rev_msg = rx.blocking_recv();
            assert!(rev_msg.is_some());

            let msg_info = rev_msg.unwrap();

            assert!(
                msg_info.clone().peer_id == internal_peerid()
                    && matches!(
                        msg_info.clone().msg.clone().as_ref(),
                        ConsensusMessageType::ProposalMessage(p)
                    )
            );
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

    #[test]
    fn is_proposer() {
        let m_latest_block_state = MockLatestBlockState::new();
        let mut m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        let addr_1 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
        let addr_2 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let addr_3 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];

        let val_1 = Validator {
            address: addr_1,
            voting_power: 1,
            proposer_priority: 1,
        };
        let val_2 = Validator {
            address: addr_2,
            voting_power: 2,
            proposer_priority: 2,
        };
        let val_3 = Validator {
            address: addr_3,
            voting_power: 3,
            proposer_priority: 3,
        };
        let val_set = ValidatorSet {
            validators: vec![val_1, val_2, val_3],
            proposer: None,
            total_voting_power: 6,
        };

        // set ADDR_2 for our priv validator
        m_priv_validator.expect_get_address().return_const(addr_2);

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
                validator_set.is_some_and(|vs| vs.proposer.is_some_and(|p| p.address.eq(&addr_3)))
            );
        } else {
            panic!("could not lock round state for arrangement")
        }
    }

    #[test]
    fn propose_timeout_send_prevote_for_nil() {
        let m_latest_block_state = MockLatestBlockState::new();
        let m_priv_validator = MockPrivValidator::new();
        let m_block_operations = MockBlockOperations::new();
        let m_block_executor = MockBlockExecutor::new();
        let m_evidence_pool = MockEvidencePool::new();

        let mut cs = ConsensusStateImpl::new(
            ConsensusConfig::new_default(),
            Arc::new(Box::new(m_latest_block_state)),
            Arc::new(Box::new(m_priv_validator)),
            Arc::new(Box::new(m_block_operations)),
            Arc::new(Box::new(m_block_executor)),
            Arc::new(Box::new(m_evidence_pool)),
        );

        let arc_cs = Arc::new(cs);

        // arrange round state
        if let Ok(mut rs_guard) = arc_cs.clone().rs.clone().lock() {
            rs_guard.height = 1;
            rs_guard.round = 1;
            drop(rs_guard);
        } else {
            panic!("could not lock round state for arrangement")
        }

        let _thread = thread::spawn(move || {
            if let Ok(_) = arc_cs.clone().start() {
            } else {
                panic!("should not failed to start")
            }
        });

        _thread.join().unwrap();
    }
}
