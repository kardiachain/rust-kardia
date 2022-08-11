use crate::types::{
    error::{AddPeerError, UnknownChannelIdError},
    peer::{ChannelId, Message, Peer, PeerRoundState},
};
use std::error::Error;

pub const STATE_CHANNEL: u8 = 0x20;
pub const DATA_CHANNEL: u8 = 0x21;
pub const VOTE_CHANNEL: u8 = 0x22;
pub const VOTE_SET_BITS_CHANNEL: u8 = 0x23;

/**
  Trait for [Consensus Reactor](../spec/consensus#consensus-reactor)

  Methods: TODO:
  
   ```
   pub newReactor(), OnStart(), OnStop(), SwitchToConsensus()
   pub SetPrivValidator(), GetPrivValidator(), GetValidators()
   pub InitPeer(), AddPeer()
   processDataCh(), processVoteCh(), processVoteSetBitsCh()
   pub handleMessage(), handleStateMessage(), handleDataMessage(), handleVoteMessage(), handleVoteSetBitsMessage()
   broadcastNewRoundStepMessage(), broadcastNewValidBlockMessage(), broadcastHasVoteMessage(), subscribeToBroadcastEvents()
   ```
*/
pub trait ConsensusReactor {
    fn new() -> Self;
    fn switch_to_consensus() -> ();
    fn set_priv_validator() -> ();
    fn get_priv_validator() -> ();
    fn get_validators() -> ();

    fn add_peer(peer: Peer) -> Result<(), Box<dyn Error>>;
    fn remove_peer(peer: Peer) -> Result<(), Box<dyn Error>>;
    fn receive(ch_id: ChannelId, src: Peer, msg: Message) -> Result<(), Box<dyn Error>>;
}

pub struct ConsensusReactorImpl {}

impl ConsensusReactor for ConsensusReactorImpl {
    fn new() -> Self {
        todo!()
    }

    fn add_peer(peer: Peer) -> Result<(), Box<dyn Error>> {
        let mut lock = peer.ps.try_lock();
        if let Ok(ref mut ps) = lock {
            // ensure peer round state is fresh
            ps.prs = PeerRoundState::new();
            Ok(())
        } else {
            Err(AddPeerError.into())
        }
    }

    fn remove_peer(peer: Peer) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn receive(ch_id: ChannelId, src: Peer, msg: Message) -> Result<(), Box<dyn Error>> {
        match ch_id {
            STATE_CHANNEL => Ok(()),
            DATA_CHANNEL => Ok(()),
            VOTE_CHANNEL => Ok(()),
            VOTE_SET_BITS_CHANNEL => Ok(()),
            _ => Err(UnknownChannelIdError.into()),
        }
    }

    fn switch_to_consensus() -> () {
        todo!()
    }

    fn set_priv_validator() -> () {
        todo!()
    }

    fn get_priv_validator() -> () {
        todo!()
    }

    fn get_validators() -> () {
        todo!()
    }
}
