use std::error::Error;

use kai_proto::consensus::Message as ConsensusMessageProto;
use prost::Message;

use crate::types::{
    error::{AddPeerError, DecodeProtoError, UnknownChannelIdError, UnknownMessageTypeError},
    messages::{msg_from_proto, Message as InternalConsensusMessage},
    peer::{ChannelId, Message as PeerMessage, Peer, PeerRoundState},
};
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
    fn receive(&self, ch_id: ChannelId, src: Peer, msg: PeerMessage) -> Result<(), Box<dyn Error>>;
}

#[derive(Default)]
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

    fn receive(&self, ch_id: ChannelId, src: Peer, msg: PeerMessage) -> Result<(), Box<dyn Error>> {
        match ConsensusReactorImpl::decode_msg(msg.as_slice()) {
            Ok(decoded_msg) => match ch_id {
                STATE_CHANNEL => self.handle_state_message(decoded_msg),
                DATA_CHANNEL => self.handle_data_message(decoded_msg),
                VOTE_CHANNEL => self.handle_vote_message(decoded_msg),
                VOTE_SET_BITS_CHANNEL => self.handle_vote_set_bits_message(decoded_msg),
                _ => Err(UnknownChannelIdError.into()),
            },
            Err(err) => Err(Box::new(err)),
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

impl ConsensusReactorImpl {
    fn decode_msg(bz: &[u8]) -> Result<Box<dyn InternalConsensusMessage>, DecodeProtoError> {
        if let Ok(proto_msg) = ConsensusMessageProto::decode(bz) {
            msg_from_proto(proto_msg)
        } else {
            Err(DecodeProtoError.into())
        }
    }

    fn handle_state_message(&self, msg: Box<dyn InternalConsensusMessage>) -> Result<(), Box<dyn Error>> {
        match msg {
            NewRoundStepMessage => {
                // TODO: handle this message
                Ok(())
            }
            NewValidBlockMessage => {
                // TODO: handle this message
                Ok(())
            }
            HasVoteMessage => {
                // TODO: handle this message
                Ok(())
            }
            VoteSetMaj23Message => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(UnknownMessageTypeError.into()),
        }
    }
    fn handle_data_message(&self, msg: Box<dyn InternalConsensusMessage>) -> Result<(), Box<dyn Error>> {
        match msg {
            ProposalMessage => {
                // TODO: handle this message
                Ok(())
            }
            ProposalPOLMessage => {
                // TODO: handle this message
                Ok(())
            }
            BlockPartMessage => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(UnknownMessageTypeError.into()),
        }
    }
    fn handle_vote_message(&self, msg: Box<dyn InternalConsensusMessage>) -> Result<(), Box<dyn Error>> {
        match msg {
            VoteMessage => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(UnknownMessageTypeError.into()),
        }
    }
    fn handle_vote_set_bits_message(&self, msg: Box<dyn InternalConsensusMessage>) -> Result<(), Box<dyn Error>> {
        match msg {
            VoteSetBitsMessage => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(UnknownMessageTypeError.into()),
        }
    }
}
