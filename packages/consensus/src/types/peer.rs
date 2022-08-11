use std::sync::{Arc, Mutex};

pub type ChannelId = u8;
pub type Message = Vec<u8>;
pub type PeerId = String;

pub struct Peer {
    pub id: PeerId,
    /**
        peer state
     */
    pub ps: Arc<Mutex<PeerState>>,
}

impl Peer {
    pub fn new(id: PeerId) -> Self {
        Self {
            id,
            ps: Arc::new(Mutex::new(PeerState::new())),
        }
    }
}

pub struct PeerState {
    /**
       peer round state
    */
    pub prs: PeerRoundState,
}

impl PeerState {
    pub fn new() -> Self {
        Self {
            prs: PeerRoundState::new(),
        }
    }
}

pub struct PeerRoundState {}

impl PeerRoundState {
    pub fn new() -> Self {
        Self {}
    }
}
