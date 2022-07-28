- [Gossip protocol specification](#gossip-protocol-specification)
  - [Models](#models)
    - [`PeerRoundState`](#peerroundstate)
    - [BlockPartMessage](#blockpartmessage)
  - [Process gossiping](#process-gossiping)
    - [Process proposal gossiping](#process-proposal-gossiping)
    - [Process votes gossiping](#process-votes-gossiping)
    - [Misconcellous](#misconcellous)

# Gossip protocol specification

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