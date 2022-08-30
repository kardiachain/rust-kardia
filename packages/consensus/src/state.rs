use crate::types::{
    config::ConsensusConfig,
    messages::{ConsensusMessage, MessageInfo},
    round_state::RoundState,
};
use kai_types::{
    block,
    block_operations::BlockOperations,
    consensus::{executor::BlockExecutor, state::LatestBlockState},
    evidence::EvidencePool,
};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{Receiver, Sender};

const MSG_QUEUE_SIZE: usize = 1000;

pub trait ConsensusState: Debug + Send + Sync + 'static {
    fn get_config(&self) -> Arc<ConsensusConfig>;
    fn get_rs(&self) -> Option<RoundState>;
    fn get_block_operations(&self) -> Arc<Box<dyn BlockOperations>>;
    fn send_peer_msg_chan(&self, msg_info: MessageInfo);
    fn send_internal_msg_chan(&self, msg_info: MessageInfo);
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
    pub state: Arc<Box<dyn LatestBlockState>>,
    pub block_operations: Arc<Box<dyn BlockOperations>>,
    pub block_exec: Arc<Box<dyn BlockExecutor>>,
    pub evidence_pool: Arc<Box<dyn EvidencePool>>,
    // pub votes: HeightVoteSet;
}

impl ConsensusStateImpl {
    pub fn new(
        config: ConsensusConfig,
        state: Box<dyn LatestBlockState>,
        block_operations: Box<dyn BlockOperations>,
        block_exec: Box<dyn BlockExecutor>,
        evidence_pool: Box<dyn EvidencePool>,
    ) -> Self {
        Self {
            config: Arc::new(config),
            state: Arc::new(state),
            block_operations: Arc::new(block_operations),
            block_exec: Arc::new(block_exec),
            evidence_pool: Arc::new(evidence_pool),
            rs: Arc::new(Mutex::new(RoundState::new())),
            peer_msg_chan: tokio::sync::mpsc::channel(MSG_QUEUE_SIZE),
            internal_msg_chan: tokio::sync::mpsc::channel(MSG_QUEUE_SIZE),
        }
    }
}

impl ConsensusState for ConsensusStateImpl {
    fn get_config(&self) -> Arc<ConsensusConfig> {
        self.config.clone()
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

    fn get_block_operations(&self) -> Arc<Box<dyn BlockOperations>> {
        self.block_operations.clone()
    }
}
