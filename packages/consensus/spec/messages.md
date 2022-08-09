- [Messages specification](#messages-specification)
  - [Processing messages](#processing-messages)
    - [Process `NewRoundStepMessage`](#process-newroundstepmessage)
    - [Processing +2/3 of a specific message of later round](#processing-23-of-a-specific-message-of-later-round)
    - [Processing proposal message](#processing-proposal-message)
    - [Processing proposal block part message](#processing-proposal-block-part-message)
    - [Processing vote message](#processing-vote-message)
  - [Type definitions of messages](#type-definitions-of-messages)
    - [Channels](#channels)
    - [`NewRoundStepMessage`](#newroundstepmessage)
    - [`NewValidBlockMessage`](#newvalidblockmessage)
    - [`HasVoteMessage`](#hasvotemessage)
    - [`VoteSetMaj23Message`](#votesetmaj23message)
    - [`ProposalMessage`](#proposalmessage)
    - [`ProposalPOLMessage`](#proposalpolmessage)
    - [`BlockPartMessage`](#blockpartmessage)
    - [`VoteMessage`](#votemessage)
    - [`VoteSetBitsMessage`](#votesetbitsmessage)

# Messages specification

In contrast to the gossiping which is to send data outbound. This specification describes how to process incoming messages that came from peers and type definitions of those messages.


TODO: in old implementation of `go-kardia`. There is a pubsub event that broadcast: NewRoundStepMessage, NewValidBlockMessage, HasVoteMessage [ref](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L388-L459). And this is how they are consumed by the consensus [ref](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L272-L287).

## Processing messages 
This section discusses about processes which digest messages that came both from peers.

Processing incoming messages affects either a peer state or the consensus state. Peer state updates can happen in parallel, but processing of proposals, block parts, and votes are ordered by the receiveRoutine eg. blocks on consensus state for proposals, block parts, and votes (TODO: change `receiveRoutine`, we've removed `receiveRoutine` in Rust implementation).

Referenced from [`go-kardia` implementation](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L242-L383).

### Process `NewRoundStepMessage`
Do validations:
- Validate height [ref](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L278)
- Validate for duplicates or decreases [ref](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L1259)
- Apply new round for peer state [ref](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/manager.go#L1254)
  - Update: height, round, step, start time
  - Reset: proposal, proposal block parts, proposal POL and round, prevotes nil, precommits nil
  - Update: precommits = catchup commit. TODO: make clear this one.

### Processing +2/3 of a specific message of later round
When received 2f+1 messages (proposal or votes) of `round` that is later `round_p`, this shows that current process is late, skip to `round` (upon rule 9). 

### Processing proposal message
See [processing proposal message](./proposal.md#processing-proposal-message)

### Processing proposal block part message
See [processing proposal block part message](./proposal.md#processing-proposal-block-part-message)

### Processing vote message
See [processing vote message](./vote.md#processing-vote-message)

## Type definitions of messages
### Channels
Messages are grouped into channels and sent along corresponding its `ChannelID`. In particular:

- `StateChannel` messages:
  - `NewRoundStepMessage`
  - `NewValidBlockMessage`
  - `HasVoteMessage`
  - `VoteSetMaj23Message`
- `DataChannel` messages:
  - `ProposalMessage`
  - `ProposalPOLMessage`
  - `BlockPartMessage`
- `VoteChannel` messages:
  - `VoteMessage`
- `VoteSetBitsChannel` messages:
  - `VoteSetBitsMessage`

### `NewRoundStepMessage`
`NewRoundStepMessage` is sent for every step taken in the ConsensusState. For every height/round/step transition
```go
type NewRoundStepMessage struct {
	Height                uint64
	Round                 uint32
	Step                  cstypes.RoundStepType
	SecondsSinceStartTime uint64
	LastCommitRound       uint32
}
```
### `NewValidBlockMessage`
`NewValidBlockMessage` is sent when a validator observes a valid block B in some round r, i.e., there is a Proposal for block B and 2/3+ prevotes for the block B in the round r. In case the block is also committed, then IsCommit flag is set to true.
```go
type NewValidBlockMessage struct {
    Height           uint64
    Round            uint32
    BlockPartsHeader types.PartSetHeader
    BlockParts       *cmn.BitArray
    IsCommit         bool
}
```
### `HasVoteMessage`
`HasVoteMessage` is sent to indicate that a particular vote has been received.
```go
type HasVoteMessage struct {
    Height uint64
    Round  uint32
    Type   kproto.SignedMsgType
    Index  uint32
}
```
### `VoteSetMaj23Message`
`VoteSetMaj23Message` is sent to indicate that a given BlockID has seen +2/3 votes.

```go
type VoteSetMaj23Message struct {
    Height  uint64
    Round   uint32
    Type    kproto.SignedMsgType
    BlockID types.BlockID
}
```
### `ProposalMessage`
`ProposalMessage` is sent when a new block is proposed.
```go
type ProposalMessage struct {
    Proposal *types.Proposal
}
```

### `ProposalPOLMessage`
`ProposalPOLMessage` is sent when a previous proposal is re-proposed.
```go
type ProposalPOLMessage struct {
    Height           uint64
    ProposalPOLRound uint32
    ProposalPOL      *cmn.BitArray
}
```

### `BlockPartMessage`
`BlockPartMessage` is sent when gossipping a piece of the proposed block.
```go
type BlockPartMessage struct {
    Height uint64
    Round  uint32
    Part   *types.Part
}
```

### `VoteMessage`
`VoteMessage` is sent when voting for a proposal (or lack thereof).
```go
type VoteMessage struct {
    Vote *types.Vote
}
```

### `VoteSetBitsMessage`
`VoteSetBitsMessage` is sent to communicate the bit-array of votes seen for the BlockID.


```go
type VoteSetBitsMessage struct {
    Height  uint64
    Round   uint32
    Type    kproto.SignedMsgType
    BlockID types.BlockID
    Votes   *cmn.BitArray
}
```