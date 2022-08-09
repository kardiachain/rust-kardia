- [Consensus specification](#consensus-specification)
  - [Kardia consensus algorithm](#kardia-consensus-algorithm)
    - [Pseudocode](#pseudocode)
      - [Terms](#terms)
  - [Consensus reactor](#consensus-reactor)
  - [Consensus state](#consensus-state)
  - [Messages flow](#messages-flow)
  - [Auxiliary processes](#auxiliary-processes)

# Consensus specification

## Kardia consensus algorithm
This consensus algorithm is based on DPoS-BFT. 

The network is composed of optionally connected _nodes_. Nodes directly connected to a particular node are called _peers_. Peers use this consensus algorithm to decide a new block to put into chain of blocks. Each peer has its own consensus process.


The consensus process in deciding the next block (at some _height_ `H`) is composed of one or many _rounds_. At each round, there is a dedicated *proposer*. The proposer has the responsibility to propose a new block and other nodes send their vote to decide. The consensus handles proposal and votes based on BFT model. 

The consensus process itself is a state machine with following states: `Propose`, `Prevote`, and `Precommit`. The so called **round** is a chain of state transitions (`Propose` -> `Prevote` -> `Precommit`). 

State transition is triggered by both internal and external factors. In particular:
- Internal factor: timeout of current state
- External factor: messages received from peers.

There are three timouts: `timeoutPropose`, `timeoutPrevote` and `timeoutPrecommit`. The timeouts prevent the algorithm from blocking (waiting for some condition to be true). Timeouts are increased every new round `r`: `timeoutX(r) = initTimeoutX + r*timeoutDelta` where `X` could be `Propose`, `Prevote` or `Precommit`, they are reset for every new height.

During the prevote step, a validator "locks" on a proposal if and only if it has +2/3 prevotes. If there is another proposal has +2/3 prevotes, the process moves to lock that new proposal. The lock is reset every new height. Locking mechanism prevents proposing 2 different proposals for the same height.

Whenever the consensus receives +2/3 prevotes of a proposal for the first time, such proposal is considered as a valid proposal. This caching method which helps to reduce the time to propose a new proposal.

Block is divided into parts and they are sent via gossip protocol. A proposer will send their summary of the proposal and proposal parts. Peers will receive: the summary, detail proposal then do the validation. If peers are not received any proposal from the proposer, peers will vote for nil in both states `Prevote` and `Precommit` then proceed to a new round.

TODO: evidence for suspicious vote

### Pseudocode

#### Terms
- `v`: value, aka. block; `id(v)`: hash of block `v`; `h_p`: height of process
- `proposer(h, r)` returns the proposer for the round `r` at height `h`.
- `upon` rule is triggered once the condition is satisfied. 
  - The condition `+2/3 <PRECOMMIT, h_p, r, id(v)>` is evaluated to `true` once there is two third of majority `PRECOMMIT` on block `v` at height `h_p` and round `r`.
  - Some of the rules ends with ”for the ﬁrst time” constraint to denote that it is triggered only the ﬁrst time a corresponding condition evaluates to true.
- The variables with index `p` are process local state variables, while variables without index p are value placeholders.
- `h_p` and `round_p` are attached to every message. Finally, a process also stores an array of decisions, `decision_p` (assumes a sequence of consensus instances, one for each height).
- `PROPOSAL` message that carries value `v`, `PREVOTE` and `PRECOMMIT` messages carry value id `id(v)`.
- A validator sends `PREVOTE(id(v))` when it evaluates `PROPOSAL(v)` is valid, otherwise `PREVOTE(nil)`.
- A validator sends `PRECOMMIT(id(v))` when it receives +2/3 `PREVOTE(id(v))`, otherwise `PRECOMMIT(nil)`.
- A validator proceeds to commit new block when it receives +2/3 `PRECOMMIT(id(v))`

```go
h_p := 0 //  height of process
round_p := 0 // round of process
step_p ∈ { propose, prevote, precommit }
decision_p[] := nil
lockedValue_p:= nil
lockedRound_p := −1 
validValue_p := nil
validRound_p := −1

upon 1: start do StartRound(0)
Function StartRound(round):
  round_p := round
  step_p := propose
  if proposer(h_p, round_p) = p then
    if validValue_p != nil then
      proposal := validValue_p
    else
      proposal := getValue()
    broadcast <PROPOSAL, h_p, round_p, proposal, validRound_p>
  else
    schedule timeoutPropose(round_p): OnTimeoutPropose(h_p, round_p) to be executed after timout

upon 2: <PROPOSAL, h_p, round_p, v, −1> from proposer(h_p, round_p) while step_p = propose do
  if valid(v) and (lockedRound_p = -1 or lockedValue_p = v) then
    broadcast <PREVOTE, h_p, round_p, id(v)>
  else
    broadcast <PREVOTE, h_p, round_p, nil>
  step_p := prevote

upon 3: <PROPOSAL, h_p, round_p, v, vr> from proposer(h_p, round_p) AND 2f+1 <PREVOTE, h_p, vr, id(v)> 
while step_p = propose and (vr >= 0 and vr < round_p ) do
  if valid(v) and (lockedRound_p <= vr or lockedValue_p = v) then 
    broadcast <PREVOTE, h_p , round_p , id(v)> 
  else
    broadcast <PREVOTE, h_p , round_p , nil>
  step_p := prevote

upon 4: 2f+1 <PREVOTE, h _p round_p, *> 
while step_p= prevote for the ﬁrst time do 
  schedule timeoutPrevote(round_p): OnTimeoutPrevote(h_p , round_p) to be executed after timout

upon 5: <PROPOSAL, h_p, round_p, v, *> from proposer(h_p, round_p) AND 2f+1 <PREVOTE, h_p, round_p, id(v)> 
while valid(v) and step_p >= prevote for the ﬁrst time do
  if step_p= prevote then
    lockedValue_p := v
    lockedRound_p := round_p
    broadcast <PRECOMMIT, h_p, round_p, id(v)>
    step_p := precommit
  validValue_p := v
  validRound_p := round_p

upon 6: 2f+1 <PREVOTE, h_p, round_p, nil>
while step_p = prevote do
  broadcast <PRECOMMIT, h_p, round_p, nil>
  step_p := precommit

upon 7: 2f+1 <PRECOMMIT, h_p, round_p, *> for the first time do
  schedule timeoutPrecommit(round_p): OnTimeoutPrecommit(h_p, round_p) to be executed after timout

upon 8: <PROPOSAL, h_p, r, v, *> from proposer(h_p, r) AND 2f+1 <PRECOMMIT, h_p, r, id(v)>
while decision_p[h_p] = nil do
  if valid(v) then
    decision_p[h_p] := v
    h_p := h_p + 1
    reset lockedRound_p, lockedValue_p, validRound_p, validValue_p to initial values
    StartRound(0)

upon 9: f+1 <*, h_p, round, *, *> with round > round_p do
  StartRound(round)

Function OnTimeoutPropose(height, round):
  if height = h_p and round = round_p and step_p = propose then
    broadcast <PREVOTE, h_p, round_p, nil>
  step_p := prevote

Function OnTimeoutPrevote(height, round):
  if height = h_p and round = round_p and step_p = prevote then
    broadcast <PRECOMMIT, h_p, round_p, nil>
  step_p ← precommit

Function OnTimeoutPrecommit(height, round):
  if height = h_p and round = round_p then 
    StartRound(round_p + 1)
```

The above consensus algorithm could be explained in more detail:
- The process starts by executing `StartRound(0)`.
- Enter `PROPOSE`: the process enters to this state either proposes a proposal or waits for a completed proposal. Then sends its prevote.
  - A proposal timeout is scheduled with `height` and `round`, function `OnTimeoutPropose(height, round)` will be executed when timeout.
  - Transition to `PREVOTE` is guaranteed by either `upon` rules (2, 3) rightaway or after above timeout.
  - Sending prevote is carried by either a `upon` rule 3 or above timeout.
- Enter `PREVOTE`: the process enters to this state listens for +2/3 prevotes to send its precommit vote. Listening for prevotes is run separately and it informs the process when +2/3 prevotes of current proposal is reached.
  - A prevote timeout is scheduled in `upon` rule 4 with `height` and `round`, function `OnTimeoutPrevote(height, round)` will be executed when timeout.
    - `upon` rule 4: +2/3 of any prevotes received => schedule timeout prevote.
  - Transition to `PRECOMMIT` is guaranteed by either `upon` rules (5, 6) rightaway or after above timeout.
    - `upon` rule 5: +2/3 of prevotes on `id(v)` => send our precommit vote for `id(v)` and transition to `PRECOMMIT` state.
    - `upon` rule 6: +2/3 of prevotes on nil => send our precommit vote for nil and transition to `PRECOMMIT` state.
  - Sending precommit vote is carried by either a `upon` rule 6 or above timeout.
- Enter `PRECOMMIT`: the process enters to this state listens for +2/3 precommits of `id(v)` to commit `v into blockchain.
  - A precommit timeout is scheduled in `upon` rule 7 with `height` and `round`, function `OnTimeoutPrecommit(hieght, round)` will be executed when timeout.
    - `upon` rule 7: +2/3 of any precommits received => schedule timeout precommit.
  - The execution of `StartRound()` (which enter `PROPOSE` state) is guaranteed by either `upon` rule 8 rightaway or after above timeout.
- The `upon` rule 9 helps it catching up the latest round of other processes.

## Consensus reactor
Consensus reactor exposes an interface `ConsensusReactor.Receive()` for `p2p` using to send messages. 

`ConsensusReactor.Receive()` processes incoming messages and might update peer state. Then it forwards them to `ConsensusState.peerMsgQueue`, the consensus engine processes each message on the queue in ordered which might make state transition. For more details, see [messages specification](./messages.md#processing-messages). 

TODO: convert model to Rust
```go

type ConsensusReactor struct {
	p2p.BaseReactor // BaseService + p2p.Switch
	conS            *ConsensusState
	waitSync        bool
	targetPending   int
	mtx             sync.RWMutex
	eventBus        *types.EventBus
}
```

## Consensus state
TODO: convert model to Rust
```go
type ConsensusState struct {
	service.BaseService

	config          *cfg.ConsensusConfig
	privValidator   types.PrivValidator // for signing votes
	blockOperations BaseBlockOperations
	blockExec       *cstate.BlockExecutor
	evpool          evidencePool // TODO(namdoh): Add mem pool.

	// internal state
	mtx sync.RWMutex
	cstypes.RoundState
	state         cstate.LatestBlockState // State until height-1.

	// State changes may be triggered by: msgs from peers,
	// msgs from ourself, or by timeouts
	peerMsgQueue     chan msgInfo
	internalMsgQueue chan msgInfo

	// we use eventBus to trigger msg broadcasts in the manager,
	// and to notify external subscribers, eg. through a websocket
	eventBus *types.EventBus

	// For tests where we want to limit the number of transitions the state makes
	nSteps int

	// a Write-Ahead Log ensures we can recover from any kind of crash
	// and helps us avoid signing conflicting votes
	wal          WAL
	replayMode   bool // so we don't log signing errors during replay
	doWALCatchup bool // determines if we even try to do the catchup

	// Synchronous pubsub between consensus state and manager.
	// State only emits EventNewRoundStep, EventVote and EventProposalHeartbeat
	evsw kevents.EventSwitch

	// closed when we finish shutting down
	done chan struct{}
}
```
## Messages flow
Consensus reactor exposes an interface for sending messages to.

Incoming messages came from network layer (p2p package),
 
## Auxiliary processes
These auxiliary processes work separately from the consensus process. They feed the consensus process by data (proposal, votes). They are grouped as following:
- Proposal
  - [Deciding proposal](./proposal.md#deciding-proposal)
  - [Processing proposal message](./proposal.md#processing-proposal-message)
  - [Processing proposal block part message](./proposal.md#processing-proposal-block-part-message)
- Processing incoming messages (proposal or votes) and check `upon` rules
  - [Processing messages](./messages.md#processing-messages)
- commit, apply new block
  - [Commit specification](./commit.md#commit-specification)
- gossip protocol
  - [Gossip protocol specification](./gossiping.md#gossip-protocol-specification)