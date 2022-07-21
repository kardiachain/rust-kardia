/**
 * This file defines consensus reactor
 * Inspired from reactor.go of Tendermint
 * https://github.com/tendermint/tendermint/blob/master/internal/consensus/reactor.go
 * 
 * Reactor:
 * - base service
 * - State: DPoS state, Tendermint
 * - RoundState: BFT state
 * - PeerState: stores state of peers for gossiping
 * - peerEvents
 * 
 * Methods:
 * - pub newReactor(), OnStart(), OnStop(), SwitchToConsensus()
 * - pub SetPrivValidator(), GetPrivValidator(), GetValidators()
 * - pub InitPeer(), AddPeer()
 * - processDataCh(), processVoteCh(), processVoteSetBitsCh()
 * - pub handleMessage(), handleStateMessage(), handleDataMessage(), handleVoteMessage(), handleVoteSetBitsMessage()
 * - broadcastNewRoundStepMessage(), broadcastNewValidBlockMessage(), broadcastHasVoteMessage(), subscribeToBroadcastEvents()
 */