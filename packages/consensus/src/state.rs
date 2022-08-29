use crate::types::{
    config::ConsensusConfig,
    messages::{ConsensusMessage, MessageInfo},
    round_state::{RoundState},
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
    // pub votes: HeightVoteSet;
}

impl ConsensusStateImpl {
    pub fn new(config: ConsensusConfig) -> Self {
        Self {
            config: Arc::new(config),
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
}
