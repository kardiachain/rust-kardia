use std::{sync::{Arc, Mutex}, thread};

use crate::types::{
    error::ConsensusReactorError,
    messages::{msg_from_proto, ConsensusMessage},
    peer::{ChannelId, Message as PeerMessage, Peer, PeerRoundState, PeerState},
};
use kai_proto::consensus::Message as ConsensusMessageProto;
use prost::Message;
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
    fn switch_to_consensus(&self) -> ();
    fn set_priv_validator(&self) -> ();
    fn get_priv_validator(&self) -> ();
    fn get_validators(&self) -> ();
    fn add_peer(&self, peer: Arc<Peer>) -> Result<(), Box<ConsensusReactorError>>;
    fn remove_peer(&self, peer: Arc<Peer>) -> Result<(), Box<ConsensusReactorError>>;
    fn receive(
        &self,
        ch_id: ChannelId,
        src: Arc<Peer>,
        msg: PeerMessage,
    ) -> Result<(), Box<ConsensusReactorError>>;
}

#[derive(Default)]
pub struct ConsensusReactorImpl {}

impl ConsensusReactor for ConsensusReactorImpl {
    fn new() -> Self {
        Self {}
    }

    fn switch_to_consensus(&self) -> () {
        todo!()
    }

    fn set_priv_validator(&self) -> () {
        todo!()
    }

    fn get_priv_validator(&self) -> () {
        todo!()
    }

    fn get_validators(&self) -> () {
        todo!()
    }

    fn add_peer(self: &ConsensusReactorImpl, peer: Arc<Peer>) -> Result<(), Box<ConsensusReactorError>> {
        let lock = peer.ps.lock();
        if let Ok(mut ps_guard) = lock {
            // ensure peer round state is fresh
            ps_guard.prs = PeerRoundState::new();

            // TODO: start gossiping threads

            Ok(())
        } else {
            Err(Box::new(ConsensusReactorError::AddPeerError(String::from(
                "lock failed: peer state has been poisoned",
            ))))
        }
    }

    fn remove_peer(&self, peer: Arc<Peer>) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }

    fn receive(
        &self,
        ch_id: ChannelId,
        src: Arc<Peer>,
        msg: PeerMessage,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match ConsensusReactorImpl::decode_msg(msg.as_slice()) {
            Ok(decoded_msg) => match ch_id {
                STATE_CHANNEL => {
                    self.handle_state_message(src, decoded_msg)
                }
                DATA_CHANNEL => self.handle_data_message(src, decoded_msg),
                VOTE_CHANNEL => self.handle_vote_message(src, decoded_msg),
                VOTE_SET_BITS_CHANNEL => self.handle_vote_set_bits_message(src, decoded_msg),
                _ => Err(Box::new(ConsensusReactorError::UnknownChannelIdError(
                    ch_id,
                ))),
            },
            Err(err) => Err(err),
        }
    }
}

impl ConsensusReactorImpl {
    fn decode_msg(bz: &[u8]) -> Result<Box<ConsensusMessage>, Box<ConsensusReactorError>> {
        if let Ok(proto_msg) = ConsensusMessageProto::decode(bz) {
            msg_from_proto(proto_msg)
        } else {
            Err(Box::new(ConsensusReactorError::DecodeProtoError))
        }
    }

    fn handle_state_message(
        &self,
        src: Arc<Peer>,
        msg: Box<ConsensusMessage>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match *msg {
            ConsensusMessage::NewRoundStepMessage(_msg) => {
                thread::spawn(move || {
                    if let Ok(mut ps_guard) = Arc::clone(&src.ps).lock() {
                        ps_guard.apply_new_round_step_message(_msg);
                        Ok(())
                    } else {
                        Err(Box::new(ConsensusReactorError::LockFailed("peer state".to_string())))
                    }
                }).join().unwrap()
            }
            ConsensusMessage::NewValidBlockMessage(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            ConsensusMessage::HasVoteMessage(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            ConsensusMessage::VoteSetMaj23Message(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    fn handle_data_message(
        &self,
        src: Arc<Peer>,
        msg: Box<ConsensusMessage>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match *msg {
            ConsensusMessage::ProposalMessage(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            ConsensusMessage::ProposalPOLMessage(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            ConsensusMessage::BlockPartMessage(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    fn handle_vote_message(
        &self,
        src: Arc<Peer>,
        msg: Box<ConsensusMessage>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match *msg {
            ConsensusMessage::VoteMessage(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    fn handle_vote_set_bits_message(
        &self,
        src: Arc<Peer>,
        msg: Box<ConsensusMessage>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match *msg {
            ConsensusMessage::VoteSetBitsMessage(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};

    use crate::{
        reactor::{ConsensusReactor, ConsensusReactorImpl, STATE_CHANNEL},
        types::{
            messages::{Message, NewRoundStepMessage},
            peer::Peer,
            round::RoundStep,
        },
    };
    use kai_proto::consensus::Message as ConsensusMessageProto;
    use prost::Message as ProstMessage;

    #[test]
    fn handle_new_round_step_msg() {
        // arrange
        let reactor: ConsensusReactorImpl = ConsensusReactorImpl::new();
        let m = NewRoundStepMessage {
            height: 1,
            round: 1,
            step: RoundStep::Propose,
            seconds_since_start_time: 1000,
            last_commit_round: 0,
        };
        let m_proto: ConsensusMessageProto = m.msg_to_proto().unwrap();
        let peer_msg = m_proto.encode_to_vec();
        let peer_id = String::from("peerid");
        let peer = Arc::new(Peer::new(peer_id));
        _ = reactor.add_peer(Arc::clone(&peer));

        // act
        let rs = reactor.receive(STATE_CHANNEL, Arc::clone(&peer), peer_msg);

        // assert
        assert!(rs.is_ok());

        let rs = thread::spawn(move || {
            if let Ok(ps_guard) = Arc::clone(&peer.ps).lock() {
                Ok(Arc::new(ps_guard.get_round_state()))
            } else {
                Err("err")
            }
        }).join();

        assert!(rs.is_ok() && rs.as_ref().unwrap().is_ok());
        let prs = Arc::clone(&rs.unwrap().unwrap());
        assert_eq!(prs.height, 1)
    }
}
