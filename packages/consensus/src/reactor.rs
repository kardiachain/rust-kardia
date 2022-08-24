use crate::{
    state::{ConsensusState, ConsensusStateImpl},
    types::{
        error::ConsensusReactorError,
        messages::{msg_from_proto, ConsensusMessage, ConsensusMessageType, MessageInfo},
        peer::{ChannelId, Message as PeerMessage, Peer, PeerRoundState},
    },
};
use kai_proto::consensus::Message as ConsensusMessageProto;
use prost::Message;
use std::sync::Arc;
use std::{result::Result::Ok, thread};

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
    fn switch_to_consensus(self: Arc<Self>) -> ();
    fn set_priv_validator(self: Arc<Self>) -> ();
    fn get_priv_validator(self: Arc<Self>) -> ();
    fn get_validators(self: Arc<Self>) -> ();
    fn add_peer(self: Arc<Self>, peer: Arc<Peer>) -> Result<(), Box<ConsensusReactorError>>;
    fn remove_peer(self: Arc<Self>, peer: Arc<Peer>) -> Result<(), Box<ConsensusReactorError>>;
    fn receive(
        self: Arc<Self>,
        ch_id: ChannelId,
        src: Arc<Peer>,
        msg: PeerMessage,
    ) -> Result<(), Box<ConsensusReactorError>>;
    fn get_cs(self: Arc<Self>) -> Arc<Box<dyn ConsensusState>>;
}

#[derive(Debug)]
pub struct ConsensusReactorImpl {
    cs: Arc<Box<dyn ConsensusState>>,
}

impl ConsensusReactor for ConsensusReactorImpl {
    fn new() -> Self {
        Self {
            cs: Arc::new(Box::new(ConsensusStateImpl::new())),
        }
    }

    fn switch_to_consensus(self: Arc<Self>) -> () {
        todo!()
    }

    fn set_priv_validator(self: Arc<Self>) -> () {
        todo!()
    }

    fn get_priv_validator(self: Arc<Self>) -> () {
        todo!()
    }

    fn get_validators(self: Arc<Self>) -> () {
        todo!()
    }

    fn add_peer(self: Arc<Self>, peer: Arc<Peer>) -> Result<(), Box<ConsensusReactorError>> {
        let lock = peer.ps.lock();
        if let Ok(mut ps_guard) = lock {
            // ensure peer round state is fresh
            ps_guard.set_prs(PeerRoundState::new());

            self.clone().gossip_data(peer.clone());
            self.clone().gossip_votes(peer.clone());
            self.clone().query_maj23(peer.clone());

            Ok(())
        } else {
            Err(Box::new(ConsensusReactorError::AddPeerError(String::from(
                "lock failed: peer state has been poisoned",
            ))))
        }
    }

    fn remove_peer(self: Arc<Self>, peer: Arc<Peer>) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }

    fn receive(
        self: Arc<Self>,
        ch_id: ChannelId,
        src: Arc<Peer>,
        msg: PeerMessage,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match ConsensusReactorImpl::decode_msg(msg.as_slice()) {
            Ok(decoded_msg) => match ch_id {
                STATE_CHANNEL => self.handle_state_message(src, decoded_msg),
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

    fn get_cs(self: Arc<Self>) -> Arc<Box<dyn ConsensusState>> {
        self.cs.clone()
    }
}

impl ConsensusReactorImpl {
    fn decode_msg(bz: &[u8]) -> Result<Arc<ConsensusMessageType>, Box<ConsensusReactorError>> {
        if let Ok(proto_msg) = ConsensusMessageProto::decode(bz) {
            msg_from_proto(proto_msg)
        } else {
            Err(Box::new(ConsensusReactorError::DecodeProtoError))
        }
    }

    fn handle_state_message(
        self: Arc<Self>,
        src: Arc<Peer>,
        msg: Arc<ConsensusMessageType>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match msg.as_ref() {
            ConsensusMessageType::NewRoundStepMessage(_msg) => {
                if let Err(e) = _msg.validate_basic() {
                    return Err(e);
                }

                if let Ok(mut ps_guard) = Arc::clone(&src.ps).lock() {
                    ps_guard.apply_new_round_step_message(_msg.clone());
                    return Ok(());
                } else {
                    return Err(Box::new(ConsensusReactorError::LockFailed(
                        "peer state".to_string(),
                    )));
                }
            }
            ConsensusMessageType::NewValidBlockMessage(_msg) => {
                if let Ok(mut ps_guard) = Arc::clone(&src.ps).lock() {
                    ps_guard.apply_new_valid_block_message(_msg.clone());
                    Ok(())
                } else {
                    Err(Box::new(ConsensusReactorError::LockFailed(
                        "peer state".to_string(),
                    )))
                }
            }
            ConsensusMessageType::HasVoteMessage(_msg) => {
                if let Ok(mut ps_guard) = Arc::clone(&src.ps).lock() {
                    ps_guard.set_has_vote(
                        _msg.height,
                        _msg.round,
                        _msg.r#type,
                        _msg.index.try_into().unwrap(),
                    );
                    Ok(())
                } else {
                    Err(Box::new(ConsensusReactorError::LockFailed(
                        "peer state".to_string(),
                    )))
                }
            }
            ConsensusMessageType::VoteSetMaj23Message(_msg) => {
                // TODO: handle this message
                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    fn handle_data_message(
        self: Arc<Self>,
        src: Arc<Peer>,
        msg: Arc<ConsensusMessageType>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match msg.as_ref() {
            ConsensusMessageType::ProposalMessage(_msg) => {
                self.cs.send_peer_msg_chan(MessageInfo {
                    peer_id: src.id.clone(),
                    msg: msg.clone(),
                });
                if let Ok(mut ps_guard) = Arc::clone(&src.ps).lock() {
                    ps_guard.set_has_proposal(_msg.clone());
                    Ok(())
                } else {
                    Err(Box::new(ConsensusReactorError::LockFailed(
                        "peer state".to_string(),
                    )))
                }
            }
            ConsensusMessageType::ProposalPOLMessage(_msg) => {
                if let Ok(mut ps_guard) = Arc::clone(&src.ps).lock() {
                    ps_guard.apply_proposal_pol_message(_msg.clone());
                    Ok(())
                } else {
                    Err(Box::new(ConsensusReactorError::LockFailed(
                        "peer state".to_string(),
                    )))
                }
            }
            ConsensusMessageType::BlockPartMessage(_msg) => {
                self.cs.send_peer_msg_chan(MessageInfo {
                    peer_id: src.id.clone(),
                    msg: msg.clone(),
                });
                if let Ok(mut ps_guard) = Arc::clone(&src.ps).lock() {
                    ps_guard.set_has_proposal_block_part(_msg.clone());
                    Ok(())
                } else {
                    Err(Box::new(ConsensusReactorError::LockFailed(
                        "peer state".to_string(),
                    )))
                }
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    fn handle_vote_message(
        self: Arc<Self>,
        src: Arc<Peer>,
        msg: Arc<ConsensusMessageType>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match msg.as_ref() {
            ConsensusMessageType::VoteMessage(_msg) => {
                if let Some(vote) = _msg.vote.clone() {
                    self.cs.send_peer_msg_chan(MessageInfo {
                        peer_id: src.id.clone(),
                        msg: msg.clone(),
                    });
                    if let Ok(mut ps_guard) = Arc::clone(&src.ps).lock() {
                        ps_guard.set_has_vote(
                            vote.height,
                            vote.round,
                            vote.r#type,
                            vote.validator_index.try_into().unwrap(),
                        );
                        Ok(())
                    } else {
                        Err(Box::new(ConsensusReactorError::LockFailed(
                            "peer state".to_string(),
                        )))
                    }
                } else {
                    Ok(())
                }
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    fn handle_vote_set_bits_message(
        self: Arc<Self>,
        src: Arc<Peer>,
        msg: Arc<ConsensusMessageType>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match msg.as_ref() {
            ConsensusMessageType::VoteSetBitsMessage(_msg) => {
                // TODO: get height, votes of consensus state
                // let height = 0;
                // let votes: HeightVoteSet = self.s;

                // if height == msg.Height {
                //     var ourVotes *cmn.BitArray
                //     switch msg.Type {
                //     case kproto.PrevoteType:
                //         ourVotes = votes.Prevotes(msg.Round).BitArrayByBlockID(msg.BlockID)
                //     case kproto.PrecommitType:
                //         ourVotes = votes.Precommits(msg.Round).BitArrayByBlockID(msg.BlockID)
                //     default:
                //         panic("Bad VoteSetBitsMessage field Type. Forgot to add a check in ValidateBasic?")
                //     }
                //     ps.ApplyVoteSetBitsMessage(msg, ourVotes)
                // } else {
                //     ps.ApplyVoteSetBitsMessage(msg, nil)
                // }
                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    fn gossip_data(self: Arc<Self>, peer: Arc<Peer>) {
        // TODO: need to handle stop this thread when peer is dead, removed
        thread::spawn(move || {
            loop {
                // self.cs
                let rs = self.clone().get_cs().get_rs();
                let prs = peer.ps.lock().unwrap().get_prs();

                // send proposal block parts if any
                if rs
                    .proposal_block_parts_header
                    .eq(&prs.proposal_block_parts_header)
                {}

                // thread sleep with r.state.config.PeerGossipSleepDuration
                // thread::sleep(dur)
            }
        });
    }

    fn gossip_votes(self: Arc<Self>, peer: Arc<Peer>) {
        // TODO: need to handle stop this thread when peer is dead, removed

        todo!()
    }

    fn query_maj23(self: Arc<Self>, peer: Arc<Peer>) {
        // TODO: need to handle stop this thread when peer is dead, removed

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};

    use crate::{
        reactor::{ConsensusReactor, ConsensusReactorImpl, STATE_CHANNEL},
        types::{
            messages::{ConsensusMessage, NewRoundStepMessage},
            peer::Peer,
        },
    };
    use kai_proto::consensus::Message as ConsensusMessageProto;
    use kai_types::round::RoundStep;
    use prost::Message as ProstMessage;

    #[test]
    fn handle_new_round_step_msg() {
        // arrange
        let reactor = Arc::new(ConsensusReactorImpl::new());
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
        _ = reactor.clone().add_peer(Arc::clone(&peer));

        // act
        let rs = reactor
            .clone()
            .receive(STATE_CHANNEL, Arc::clone(&peer), peer_msg);

        // assert
        assert!(rs.is_ok());

        let rs = thread::spawn(move || {
            if let Ok(ps_guard) = Arc::clone(&peer.ps).lock() {
                Ok(Arc::new(ps_guard.get_prs()))
            } else {
                Err("err")
            }
        })
        .join();

        assert!(rs.is_ok() && rs.as_ref().unwrap().is_ok());
        let prs = Arc::clone(&rs.unwrap().unwrap());
        assert_eq!(prs.height, 1);
        assert_eq!(prs.height, 1);
        assert_eq!(prs.round, 1);
        assert_eq!(prs.step, RoundStep::Propose);
        assert_eq!(prs.last_commit_round, 0);
    }

    #[test]
    fn handle_new_valid_block_msg() {
        // arrange
        let reactor = Arc::new(ConsensusReactorImpl::new());
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
        _ = reactor.clone().add_peer(Arc::clone(&peer));

        // act
        let rs = reactor.receive(STATE_CHANNEL, Arc::clone(&peer), peer_msg);

        // assert
        assert!(rs.is_ok());

        let rs = thread::spawn(move || {
            if let Ok(ps_guard) = Arc::clone(&peer.ps).lock() {
                Ok(Arc::new(ps_guard.get_prs()))
            } else {
                Err("err")
            }
        })
        .join();

        assert!(rs.is_ok() && rs.as_ref().unwrap().is_ok());
        let prs = Arc::clone(&rs.unwrap().unwrap());
        assert_eq!(prs.height, 1);
        assert_eq!(prs.height, 1);
        assert_eq!(prs.round, 1);
        assert_eq!(prs.step, RoundStep::Propose);
        assert_eq!(prs.last_commit_round, 0);
    }
}
