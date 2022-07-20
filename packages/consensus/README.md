# consensus package
This package implements the DPoS-BFT consensus engine. It maintains internal state and peer state via transporting messages via `kardia-sentry` service.
Its main components are:
- `TimeoutTicker`: for processing data in round and steps
- `State`: for storing internal state of the consensus engine
- `Reactor`: handle messages from peer or internal state changes via channels (state, data, vote, votebit)
- WAL (write-ahead-log): a canonical WAL to recover from any crashes.

## Timeout Ticker
TimeoutTicker is a timer that schedules timeouts conditional on the height/round/step in the timeoutInfo.
### Models
```rust
trait TimeoutTicker {
    pub fn start() -> Result<()>;
    pub fn stop() -> ();
    pub fn is_running -> Result<bool>;
    pub fn schedule_timeout(ti: TimeoutInfo) -> Result<()>;
}
struct TimeoutInfo {
    duration: int64;
    height: uint64;
    round: uint32;
    step: RoundStepType;
}
enum RoundStepType {
    RoundStepNewHeight = 1, // Wait til CommitTime + timeoutCommit
    RoundStepNewRound, // Setup new round and go to RoundStepPropose
    RoundStepPropose, // Did propose, gossip proposal
    RoundStepPrevote, // Did prevote, gossip prevotes
    RoundStepPrevoteWait, // Did receive any +2/3 prevotes, start timeout
    RoundStepPrecommit, // Did precommit, gossip precommits
    RoundStepPrecommitWait, // Did receive any +2/3 precommits, start timeout
    RoundStepCommit, // Entered commit state machine
    // NOTE: RoundStepNewHeight acts as RoundStepCommitWait.
}
```
It has 2 important channels:
- `tick_chan`: for scheduling timeouts
- `tock_chan`: for notifying (consensus engine) timeouts with corresponding timeout info.
### `start()`
It initializes `tick_chan`, `tock_chan`, stores them in struct. Setup the timer.
### `schedule_timeout()`
It fires a new `timout_info` on `tick_chan`. Whenever a new tick comes it sets the timer with duration specified in `timeout_info`. When the timer is out, it sends `timeount_info` to `tock_chan`. `ConsensusState` listens on this channel and handle in `ConsensusState.handle_timeout()`.
The timer is interupted and replaced by new tick, tickers for old height/round/step are ignored.
### `stop()`
It stops the timer and drain if necessary.

## Consensus reactor
Methods:
```
OnStart()
OnStop()
SwitchToConsensus()
handleMessage(): listens from P2P connection and handle corresponding message.
subscribeToBroadcastEvents(): communication between react and state
// routines
updateRoundStateRoutine()
gossipDataRoutine(), gossipVotesRoutine(), queryMaj23Routine(): routines for every new P2P connection.
```
### OnStart()
OnStart starts separate go routines for each p2p Channel and listens for envelopes on each. In addition, it also listens for peer updates and handles messages on that p2p channel accordingly. The caller must be sure to execute OnStop to ensure the outbound p2p Channels are closed.
Go routines invokes by OnStart():
```
updateRoundStateRoutine()
processStateCh()
processDataCh()
processVoteCh()
processVoteSetBitsCh()
processPeerUpdates(): it listens on "peerUpdates.Updates()", then invoke processPeerUpdate()
```
`processStateCh(), processDataCh(), processVoteCh(), processVoteSetBitsCh()` are channel for P2P communications, they invoke `handleMessage()`.
### OnStop()
OnStop stops the reactor by signaling to all spawned goroutines to exit and blocking until they all exit, as well as unsubscribing from events and stopping state.
### SwitchToConsensus()
SwitchToConsensus switches from block-sync mode to consensus mode. It resets the state, turns off block-sync, and starts the consensus state-machine.
### handleMessage()
handleMessage handles an Envelope sent from a peer on a specific p2p Channel.
It will handle errors and any possible panics gracefully. A caller can handle
any error returned by sending a PeerError on the respective channel.
### processPeerUpdate()
processPeerUpdate process a peer update message. For new or reconnected peers,
we create a peer state if one does not exist for the peer, which should always
be the case, and we spawn all the relevant goroutine to broadcast messages to
the peer. During peer removal, we remove the peer for our set of peers and
signal to all spawned goroutines to gracefully exit in a non-blocking manner.

### subscribeToBroadcastEvents()
Reactor and State use `state.evsw`: EventSwitch, synchronous pubsub between consensus state and reactor. state only emits EventNewRoundStep, EventValidBlock, and EventVote
It routes message to `state.broadcastNewRoundStepMessage()`, `state.broadcastNewValidBlockMessage()` and `state.broadcastHasVoteMessage()`

## Consensus state
State handles execution of the consensus algorithm.
It processes votes and proposals, and upon reaching agreement, commits blocks to the chain and executes them against the application.
The internal state machine receives input from peers, the internal validator, and from a timer.

Bootstrapping Methods:
```
OnStart()
OnStop()
receiveRoutine()
updateToState(state)
```

Public interface for passing messages into the consensus state (mostly Consensus Reactor), possibly causing a state transition:
```
AddVote()
SetProposal()
AddProposalBlockPart()
SetProposalAndBlock()
```

Internal functions for managing the state:
```
updateHeight()
updateRoundStep()
scheduleRound0()
scheduleTimeout()
sendInternalMessage()
reconstructLastCommit()
votesFromExtendedCommit()
votesFromSeenCommit()
updateToState()
newStep()
```

### OnStart()
OnStart loads the latest state via the WAL, starts the timeout ticker (schedules first round) and receive routines.
If peerID == "", the msg is considered internal. Messages are added to the appropriate queue (peer or internal). If the queue is full, the function may block.

### updateToState()
Updates State and increments height to match that of state. It is called inside SwitchToConsensus() of Reactor, finalizeCommit() and updateStateFromStore() of the state ifself.

### receiveRoutine()
receiveRoutine handles messages which may cause state transitions.
It's argument (n) is the number of messages to process before exiting - use 0 to run forever
It keeps the RoundState and is the only thing that updates it.
Updates (state transitions) happen on timeouts, complete proposals, and 2/3 majorities.
It does as follows:
- Listen on `txNotifier.TxsAvailable()`, it handles `handleTxsAvailable()`
- Listen on `cs.peerMsgQueue`, it handles proposals, block parts, votes
- Listen on `cs.internalMsgQueue`, it handles proposals, block parts, votes
- Listen on `tockChan`, it handles timeouts.

### handleTimout()
It listens for `timeout_info` on the `tock_chan` of the `TimeoutTicker`. 
It checks `timeout_info.step`, in case the step is:
- *RoundStepNewHeight*: invokes
  - `cs.enterNewRound()`
- *RoundStepNewRound*: invokes
  - `cs.enterPropose()`,
- *RoundStepPropose*: invokes
  - PublishEventTimeoutPropose
  - `cs.enterPrevote()`,
- *RoundStepPrevoteWait*: invokes
  - PublishEventTimeoutWait
  - `cs.enterPrecommit()`,
- *RoundStepPrecommitWait*: invokes
  - PublishEventTimeoutWait
  - `cs.enterPrecommit()`
  - `cs.enterNewRound()`.

### state functions
#### `enterNewRound()`


### finalizeCommit()


## Kardia consensus algorithm
Keynotes:
- There is a bound `delta` and an instant `GST` (Global Stabilization Time) such that all communication among validators after GST is reliable and delta-timely.
- `v`: value, aka. block; `id(v)`: hash of block `v`; `h_p`: height of process
- `upon` rule is triggered once the condition is satisfied. 
  - The condition `+2/3 <PRECOMMIT, h_p, r, id(v)>` is evaluated to `true` once there is two third of majority `PRECOMMIT` on block `v` at height `h_p` and round `r`.
  - Some of the rules ends with ”for the ﬁrst time” constraint to denote that it is triggered only the ﬁrst time a corresponding condition evaluates to true.
- The variables with index `p` are process local state variables, while variables without index p are value placeholders.
- Algorithm proceeds in rounds, where each round has a decicated *proposer*. Function `proposer(h, r)` returns the proposer for the round `r` at height `h`.
- There are three timouts: `timeoutPropose`, `timeoutPrevote` and `timeoutPrecommit`. The timeouts prevent the algorithm from blocking (waiting for some condition to be true). Timeouts are increased every new round `r`: `timeoutX(r) = initTimeoutX + r*timoutDelta` where `X` could be `Propose`, `Prevote` or `Precommit`, they are reset for every new height.
- Apart from those variables, a process also stores the current consensus instance (h_p , called height in Tendermint), and the current round number (round_p ) and attaches them to every message. Finally, a process also stores an array of decisions, decision p (Tendermint assumes a sequence of consensus instances, one for each height).

### State transitions
- `PROPOSAL` message that carries value, `PREVOTE` and `PRECOMMIT` messages carry value id.
- States: `NEWHEIGHT` -> (`PROPOSAL` -> `PREVOTE` -> `PRECOMMIT`)+ -> `COMMIT`
  - A validator sends `PREVOTE(id(v))` when it evaluates `PROPOSAL(v)` is valid, otherwise `PREVOTE(nil)`.
  - A validator sends `PRECOMMIT(id(v))` when it receives +2/3 `PREVOTE(id(v))`, otherwise `PRECOMMIT(nil)`.
  - A validator proceeds to `COMMIT` when it receives +2/3 `PRECOMMIT(id(v))`
- A validator also "locks" on the most recent value `v` that has +2/3 `PREVOTE` (or before sending `PRECOMMIT(id(v))`). The lock is reset every new height. `validValue` is used to store forementioned value and `validRound` is the round `r` when `validValue` gets assigned.

Kardiachain consensus algorithm pseudocode:
```go
h_p := 0 //  height of process
round_p := 0 // round of process
step_p ∈ { propose, prevote, precommit }
decision_p [] := nil
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
  if step p= prevote then
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
  step p ← precommit

Function OnTimeoutPrecommit(height, round):
  if height = h_p and round = round_p then 
    StartRound(round_p + 1)
```

Note on `upon` rules: (TODO: Need to clarify)
- `upon CONDITIONS_1 while CONDITIONS_2 do DO_SOMETHING`: every satisfaction on `CONDITIONS_1`, if `CONDITIONS_2`  satisfies then `DO_SOMETHING`. 
- `upon CONDITIONS_1 ... for the first time do DO_SOMETHING`: execute `DO_SOMETHING` only once on first satisfaction of `CONDITIONS_1`.

The above consensus algorithm could be explained in more detail:
- The process starts by executing `StartRound(0)`. The `upon` rule 9 helps it catching up the latest round of other processes.
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
- Enter `PRECOMMIT`: the process enters to this state listens for +2/3 precommits of `id(v)` to commit `v`.
  - A precommit timeout is scheduled in `upon` rule 7 with `height` and `round`, function `OnTimeoutPrecommit(hieght, round)` will be executed when timeout.
    - `upon` rule 7: +2/3 of any precommits received => schedule timeout precommit.
  - The execution of `StartRound()` (which enter `PROPOSE` state) is guaranteed by either `upon` rule 8 rightaway or after above timeout.


