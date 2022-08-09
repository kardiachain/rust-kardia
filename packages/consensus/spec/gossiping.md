- [Gossip protocol specification](#gossip-protocol-specification)
  - [Models](#models)
    - [`PeerRoundState`](#peerroundstate)
    - [BlockPartMessage](#blockpartmessage)
  - [Process gossiping](#process-gossiping)
    - [Process proposal gossiping](#process-proposal-gossiping)
    - [Process votes gossiping](#process-votes-gossiping)
    - [Misconcellous](#misconcellous)

# Gossip protocol specification

Gossiping in Kardia node client is to sent data outbound, eg. from the local node to connected peers. For each connected peer, the consensus reactor keeps tracking of peer state for gossiping purpose. (For processing incoming messages from peers, please refer to [Messages specification](#messages-specification))

Based on a peer's state, consensus reactor gossips three main things:
- Data: proposal, proposal block parts, catchup blocks. TODO: read from [here](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L462-L554)
- Votes: broadcast missing votes based on peer state. TODO: read from [here](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L601-L671)
- +2/3: broadcast block id has been received +2/3 votes based on peer state. TODO: read from [here](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L728-L812)

Every time the consensus reactor gossiped a piece of data, then it sleeps for predefined duration (via configuration).

Note: 
## Models
### `PeerRoundState`
### BlockPartMessage

## Process gossiping
The process tracks every connected peer, it gossips three things:
- data
- votes
- any +2/3 votes for a block

### Process proposal gossiping
```
SetHasProposal
InitProposalBlockParts
SetHasProposalBlockPart
// ApplyProposalPOLMessage updates the peer state for the new proposal POL.

```

### Process votes gossiping
```
PickSendVote
EnsureVoteBitArrays
SetHasVote
// ApplyHasVoteMessage updates the peer state for the new vote.
// ApplyVoteSetBitsMessage updates the peer state for the bit-array of votes

```

### Misconcellous
```
// ApplyNewValidBlockMessage updates the peer state for the new valid block.
ensureCatchupCommitRound
ApplyNewRoundStepMessage
```