- [Messages specification](#messages-specification)
  - [Message types](#message-types)
    - [`Message`](#message)
    - [`VoteMessage`](#votemessage)
    - [`ProposalMessage`](#proposalmessage)
    - [`HasVoteMessage`](#hasvotemessage)
    - [`VoteSetMaj23Message`](#votesetmaj23message)
    - [`VoteSetBitsMessage`](#votesetbitsmessage)
    - [`BlockPartMessage`](#blockpartmessage)
    - [`NewValidBlockMessage`](#newvalidblockmessage)
  - [Processing messages](#processing-messages)
    - [Processing +2/3 of a specific message of later round](#processing-23-of-a-specific-message-of-later-round)
    - [Processing proposal message](#processing-proposal-message)
    - [Processing proposal block part message](#processing-proposal-block-part-message)
    - [Processing vote message](#processing-vote-message)

# Messages specification

## Message types

### `Message`

### `VoteMessage`

### `ProposalMessage`

### `HasVoteMessage`

### `VoteSetMaj23Message`

### `VoteSetBitsMessage`

### `BlockPartMessage`

### `NewValidBlockMessage`

## Processing messages 
This section discusses about processes which digest messages (proposal or votes) that came both from peers and consensus engine itself.

Every message type has its own way to process. The difference is described as below.

### Processing +2/3 of a specific message of later round
When received 2f+1 messages (proposal or votes) of `round` that is later `round_p`, this shows that current process is late, skip to `round` (upon rule 9). 

### Processing proposal message
See [processing proposal message](./proposal.md#processing-proposal-message)

### Processing proposal block part message
See [processing proposal block part message](./proposal.md#processing-proposal-block-part-message)

### Processing vote message
See [processing vote message](./vote.md#processing-vote-message)
