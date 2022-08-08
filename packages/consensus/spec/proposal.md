- [Proposal specification](#proposal-specification)
  - [Models](#models)
    - [`Proposal`](#proposal)
  - [Validations](#validations)
  - [Sign proposal](#sign-proposal)
  - [Functions](#functions)
    - [Deciding proposal](#deciding-proposal)
    - [Validating proposal](#validating-proposal)
    - [Processing proposal message](#processing-proposal-message)
    - [Processing proposal block part message](#processing-proposal-block-part-message)

# Proposal specification

A proposal is a block proposal for the consensus process to be put in the blockchain, it contains a list of transactions (or empty if there is no transaction).

The consensus engine stores the proposal under following fields: `Proposal`, `ProposalBlock` and `ProposalBlockParts`.

The full proposal are splited into parts and gossiped to peers to reduce the burden of transmission for the proposer. A proposer broadcasts their **signed** proposal by `Proposal` message as a summary. Peers receive the `Proposal`, then it will ask for the detail proposal (eg. `ProposalBlock`, `ProposalBlockParts`) via gossiping protocol. 

Proposal message contains following information: `height, round, timestamp, signature, POLRound and POLBlockId`.
The consensus received a proposal message (either from a peer or the consensus engine itself).

## Models
```
Proposal {
    Height // current height
    Round // current round
    Timestamp // creation time
    POLRound // round where proposer locked on
    POLBlockID // block id where proposer locked on
}
```
## validations
## Functions
### Deciding proposal
TODO:
The process which decides a new proposal.

Preconditions
- The consensus engine is the proposer at this round, it  triggered this process for deciding a proposal.
- The consensus engine state contains: `height, round, proof-of-lock round & block, valid block` 

Process:
- Deciding block
  - If the consensus engine already knew a valid block (+2/3 prevotes), use that valid block. 
  - Otherwise, it proposes a new block by collecting txs from txpool. If the txpool isn't ready yet, it keeps waiting. The consensus engine will automatically move to new round if the proposing process is timeout.
- Making proposal: 
  - Creating a new proposal with necessary information: height, round, timestamp, proof-of-lock round, block id
    - proof-of-lock round is the round where proposer locked on
    - block id is the identity of proposal block
  - Sign the proposal

### Validating proposal
TODO:
The process validates a proposal. It is triggered by the consensus engine (after receiving a complete proposal). 

### Processing proposal message
TODO:
Proposal message contains following information: `height, round, timestamp, signature, POLRound and POLBlockId`.
The consensus received a proposal message (either from a peer or the consensus engine itself).
Only the proposal that satisfies following validations is accepted:
- `proposal.height = height_p`, `proposal.round == round_p`
- `-1 <= POLRound < round_p`
- `proposal` must be proposed by `proposer(height, round)`
- `signature` is valid

Note that the proposal does not include the content of the proposal block. There is another process to receive part of the block.

### Processing proposal block part message
TODO:
Proposal block are splitted into parts. They are sent part by part. 
This process receives block part message.
Until the process receives complete proposal, it:
- broadcast event complete proposal (for gossiping)
- do check validity of the proposal, its votes. See `upon` rules (2,3,5,8).
