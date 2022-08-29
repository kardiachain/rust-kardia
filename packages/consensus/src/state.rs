use crate::types::{
    messages::{ConsensusMessage, MessageInfo},
    round_state::{RoundState, RoundStateImpl}, config::ConsensusConfig,
};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{Receiver, Sender};

const MSG_QUEUE_SIZE: usize = 1000;

pub trait ConsensusState: Debug + Send + Sync + 'static {
    fn get_rs(&self) -> RoundStateImpl;
    fn send_peer_msg_chan(&self, msg_info: MessageInfo);
    fn send_internal_msg_chan(&self, msg_info: MessageInfo);
}

#[derive(Debug)]
pub struct ConsensusStateImpl {
    pub config: Arc<ConsensusConfig>,
    pub rs: Arc<Mutex<RoundStateImpl>>,
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
            rs: Arc::new(Mutex::new(RoundStateImpl::new())),
            peer_msg_chan: tokio::sync::mpsc::channel(MSG_QUEUE_SIZE),
            internal_msg_chan: tokio::sync::mpsc::channel(MSG_QUEUE_SIZE),
        }
    }
}

impl ConsensusState for ConsensusStateImpl {
    fn get_rs(&self) -> RoundStateImpl {
        self.rs.clone()
    }

    fn send_peer_msg_chan(&self, msg_info: MessageInfo) {
        todo!()
    }

    fn send_internal_msg_chan(&self, msg_info: MessageInfo) {
        todo!()
    }
}
