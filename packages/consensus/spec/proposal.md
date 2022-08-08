- [Proposal specification](#proposal-specification)
  - [Models](#models)
    - [`Proposal`](#proposal)
    - [`ProposalBlock`](#proposalblock)
    - [`ProposalBlockParts`](#proposalblockparts)
  - [Functions](#functions)
    - [Deciding proposal](#deciding-proposal)
      - [Sign proposal](#sign-proposal)
    - [Processing proposal message](#processing-proposal-message)
    - [Processing proposal block part message](#processing-proposal-block-part-message)

# Proposal specification

A proposal is a block proposal for the consensus process to be put in the blockchain, it contains a list of transactions (or empty if there is no transaction).

The consensus engine stores the proposal under following fields: `Proposal`, `ProposalBlock` and `ProposalBlockParts`.

The full proposal are splited into parts and gossiped to peers to reduce the burden of transmission for the proposer. A proposer broadcasts their **signed** proposal by `Proposal` message as a summary. Once peers receive the `Proposal`, then they will ask for the detail proposal (ie. `ProposalBlock`, `ProposalBlockParts`) via gossiping protocol. 

Proposal message contains following information: `height, round, timestamp, signature, POLRound and POLBlockId`.

The consensus receives a proposal message (either from a peer or the consensus engine itself), validates (only for proposal came from a peer) and votes on that proposal.

## Models
This section describes data structures of a proposal.
### `Proposal`
```
Proposal {
    Height // current height
    Round // current round
    POLRound // round where proposer locked on
    POLBlockID // block id where proposer locked on
    Timestamp // creation time
    Signature // signature of the proposer
}
```
### `ProposalBlock`


### `ProposalBlockParts`


## Functions
### Deciding proposal
The process which decides a new proposal.

Preconditions:
- The consensus engine is the proposer at this round, it  triggered this process for deciding a proposal.
- The consensus engine state contains: `height, round, proof-of-lock round & block, valid block` 

Process:
- Deciding block proposal:
  - If the consensus engine already knew a valid block (+2/3 prevotes), use that valid block. 
  - Otherwise, it delegates proposal making request the request to the consensus engine's block operations. The consensus engine will automatically move to new round if the proposing process is timed out.
- Making proposal: 
  - Creating a new proposal with necessary information: height, round, timestamp, proof-of-lock round, block id, commit
    - proof-of-lock round is the round where proposer locked on
    - block id is the identity of proposal block
    - commit contains consensus engine's precommits of previous height. In case of height 1, precommits is [constructed with value `empty`](https://github.com/kardiachain/go-kardia/blob/7b90a657494230b99afb54135882cf2f78ec0395/consensus/state.go#L1526)
  - Sign the proposal

#### Sign proposal
The `Proposal` must be signed by the proposer. The `Proposal` is converted to proto-encoding for signing.

TODO: proposal.go/64, it converts proposal to proto bytes -> sign with that bytes

### Processing proposal message
TODO:
Proposal message contains following information: `height, round, timestamp, signature, POLRound and POLBlockId`.
The consensus received a proposal message (either from a peer or the consensus engine itself).
Only the proposal that satisfies following validations is accepted:
- `proposal.height == height_p`, `proposal.round == round_p`
- `-1 <= POLRound < round_p`
- `proposal` must be proposed by `proposer(height, round)`
- `signature` is valid

Note that the proposal does not include the content of the proposal block. There is another process to receive part of the block.

### Processing proposal block part message
Proposal block are splitted into parts and gossiped.

This process receives block part message and adds every received part to consensus state `ProposalBlockParts`.

Only block part that satisfies following validations is accepted:
- `part.height == height_p` (1)

Once block part is added, the process check that whether it has received a complete proposal then it does:
- broadcast event complete proposal (ie. for whom are missing proposal, they know who can beg for the proposal)  
- do check validity of the proposal (**), if the proposal is:
  - valid: then feed `upon` rules (2,3,5,8) by sending the complete proposal via a (tx,rx) channel of consensus. Those rules listen on that channel, check on their rule and know what to decide next.
  - invalid: then terminate processing, consensus engine will automatically move to prevote step when it's timed out.

NOTE: 
1. we accept round mismatch 
2. since proposal is a block, by checking its validity also mean checking validity of that block. Refer to `BlockExecutor.ValidateBlock()`.