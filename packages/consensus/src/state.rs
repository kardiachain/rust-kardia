use crate::types::{
    config::ConsensusConfig,
    error::{ConsensusReactorError, ConsensusStateError},
    messages::{
        BlockPartMessage, ConsensusMessage, ConsensusMessageType, MessageInfo, ProposalMessage,
        VoteMessage,
    },
    peer::interal_peerid,
    round_state::RoundState,
};
use kai_types::{
    block::{Block, BlockId},
    block_operations::BlockOperations,
    consensus::{executor::BlockExecutor, state::LatestBlockState},
    evidence::EvidencePool,
    part_set::PartSet,
    proposal::Proposal,
    round::RoundStep,
    types::SignedMsgType, priv_validator::{PrivValidator, self}, timestamp,
};
use mockall::automock;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex, MutexGuard},
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
    fn send_in_msg_chan(&self, msg_info: MessageInfo) -> Result<(), TrySendError<MessageInfo>>;

    fn update_to_state(&self, state: Arc<Box<dyn LatestBlockState>>);
    fn start(&self) -> Result<(), Box<ConsensusReactorError>>;
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
    pub in_msg_chan: ConsensusMsgChan<MessageInfo>,
    pub out_msg_chan: ConsensusMsgChan<MessageInfo>,
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

impl ConsensusStateImpl {
    pub fn new(
        config: ConsensusConfig,
        state: Arc<Box<dyn LatestBlockState>>,
        priv_validator: Arc<Box<dyn PrivValidator>>,
        block_operations: Arc<Box<dyn BlockOperations>>,
        block_exec: Arc<Box<dyn BlockExecutor>>,
        evidence_pool: Arc<Box<dyn EvidencePool>>,
    ) -> Self {
        let new_msg_chan = || {
            let (tx, rx) = tokio::sync::mpsc::channel(MSG_QUEUE_SIZE);
            ConsensusMsgChan { tx, rx }
        };

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
            in_msg_chan: new_msg_chan(),
            out_msg_chan: new_msg_chan(),
        }
    }
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

    fn send_in_msg_chan(&self, msg_info: MessageInfo) -> Result<(), TrySendError<MessageInfo>> {
        self.in_msg_chan.tx.try_send(msg_info)
    }

    fn start(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }

    fn update_to_state(&self, state: Arc<Box<dyn LatestBlockState>>) {
        todo!()
    }
}

impl ConsensusStateImpl {
    async fn process_in_msg_chan(&mut self) {
        while let Some(msg_info) = &self.in_msg_chan.rx.recv().await {
            let msg = msg_info.msg.clone();
            match msg.as_ref() {
                ConsensusMessageType::ProposalMessage(_msg) => {
                    log::debug!("set proposal: proposal={:?}", _msg.clone());
                    if let Err(e) = self.set_proposal(_msg.clone()) {
                        log::error!(
                            "set proposal failed: peerid={} msg={:?}, err={}",
                            msg_info.peer_id,
                            _msg.clone(),
                            e
                        )
                    }
                }
                ConsensusMessageType::BlockPartMessage(_msg) => {
                    log::debug!("set block part: blockpart={:?}", _msg.clone());
                    if let Err(e) = self.add_proposal_block_part(_msg.clone()) {
                        log::error!(
                            "set block part failed: peerid={} msg={:?}, err={}",
                            msg_info.peer_id,
                            _msg.clone(),
                            e
                        )
                    }
                }
                ConsensusMessageType::VoteMessage(_msg) => {
                    log::debug!("set vote: vote={:?}", _msg.clone());
                    if let Err(e) = self.try_add_vote(_msg.clone()) {
                        log::error!(
                            "set vote failed: peerid={} msg={:?}, err={}",
                            msg_info.peer_id,
                            _msg.clone(),
                            e
                        )
                    }
                }
                _ => {
                    log::error!("unknown msg type: type={:?}", msg)
                }
            };
        }
    }

    fn set_proposal(&self, msg: ProposalMessage) -> Result<(), ConsensusStateError> {
        todo!()

        // TODO: check upons rule
    }

    fn add_proposal_block_part(&self, msg: BlockPartMessage) -> Result<(), ConsensusStateError> {
        todo!()

        // TODO: check upons rule
    }

    fn try_add_vote(&self, msg: VoteMessage) -> Result<(), ConsensusStateError> {
        todo!()

        // TODO: check upons rule
    }

    fn on_timeout_propose(&self, height: u64, round: u32) {
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

    fn on_timeout_prevote(&self, height: u64, round: u32) {
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

    fn on_timeout_precommit(&self, height: u64, round: u32) {
        if let Ok(rs_guard) = self.rs.clone().lock() {
            if height == rs_guard.height && round == rs_guard.round {
                // TODO: self.start_round(rs_guard.round + 1);
            }
            drop(rs_guard);
        } else {
            log::trace!("lock round state failed")
        }
    }

    fn start_round(&self, round: u32) {
        if let Ok(mut rs_guard) = self.rs.clone().lock() {
            rs_guard.round = round;
            rs_guard.step = RoundStep::Propose;

            if rs_guard.is_proposer() {
                self.decide_proposal(rs_guard.clone());
            } else {
                //     schedule timeoutPropose(round_p): OnTimeoutPropose(h_p, round_p) to be executed after timout
            }

            drop(rs_guard);
        } else {
            log::trace!("lock round state failed")
        }
    }

    fn decide_proposal(&self, rs: RoundState) {
        let block: Option<Block>;
        let block_parts: Option<PartSet>;

        if rs.valid_block.is_some() {
            block = rs.valid_block;
            block_parts = rs.valid_block_parts;
        } else {
            (block, block_parts) = self.create_proposal_block();
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

            let priv_validator = self.priv_validator.clone();
            let chain_id = self.state.clone().get_chain_id();

            if let Ok(_) = priv_validator.clone().sign_proposal(chain_id, &mut proposal) {
                let msg_info = MessageInfo {
                    msg: Arc::new(ConsensusMessageType::ProposalMessage(ProposalMessage {
                        proposal: Some(proposal),
                    })),
                    peer_id: interal_peerid(),
                };
    
                _ = self.in_msg_chan.tx.try_send(msg_info);
            } else {
                log::error!("sign proposal failed");
                return
            }
        } else {
            log::error!("invalid block or block parts")
        }
    }

    fn create_proposal_block(&self) -> (Option<Block>, Option<PartSet>) {
        todo!()
    }
}
