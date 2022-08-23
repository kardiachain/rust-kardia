use crate::types::messages::{ConsensusMessage, MessageInfo};
use std::fmt::Debug;
use tokio::sync::mpsc::{Receiver, Sender};

const MSG_QUEUE_SIZE: usize = 1000;

pub trait ConsensusState: Debug + Send + Sync + 'static {
    fn send_peer_msg_chan(&self, msg_info: MessageInfo);
    fn send_internal_msg_chan(&self, msg_info: MessageInfo);
}

#[derive(Debug)]
pub struct ConsensusStateImpl {
    // pub votes: HeightVoteSet;
    pub peer_msg_chan: (
        Sender<Box<dyn ConsensusMessage>>,
        Receiver<Box<dyn ConsensusMessage>>,
    ),
    pub internal_msg_chan: (
        Sender<Box<dyn ConsensusMessage>>,
        Receiver<Box<dyn ConsensusMessage>>,
    ),
}

impl ConsensusStateImpl {
    pub fn new() -> Self {
        Self {
            peer_msg_chan: tokio::sync::mpsc::channel(MSG_QUEUE_SIZE),
            internal_msg_chan: tokio::sync::mpsc::channel(MSG_QUEUE_SIZE),
        }
    }
}

impl ConsensusState for ConsensusStateImpl {
    fn send_peer_msg_chan(&self, msg_info: MessageInfo) {
        todo!()
    }

    fn send_internal_msg_chan(&self, msg_info: MessageInfo) {
        todo!()
    }
}
