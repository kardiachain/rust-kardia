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


TODO: 
- Enter `PROPOSE`: the process enters to this state either proposes a proposal or waits for a complete proposal. Then sends its prevote.
  - A proposal timeout is scheduled with `height` and `round`, function `OnTimeoutPropose(height, round)` will be executed when timeout.
  - Transition to `PREVOTE` is guaranteed by either `upon` rules (lines 22, 28) or above timeout. The transition happens RIGHTAWAY.
  - Sending prevote is carried by either a `upon` rule (line 28) or above timeout.
- Enter `PREVOTE`: the process enters to this state listens for +2/3 prevotes to send its precommit vote. Listening for prevotes is run separately and it informs the process when +2/3 prevotes of current proposal is reached.
  - A prevote timeout is scheduled with `height` and `round`, function `OnTimeoutPrevote(height, round)` will be executed when timeout.
  - Transition to `PRECOMMIT` is guaranteed by either `upon` rules (line 36, 44) rightaway or after above timeout scheduled in `upon` rule (line 34).
    - `upon` rule 34: +2/3 of any prevotes received => schedule timeout prevote.
    - `upon` rule 36: +2/3 of prevotes on `id(v)` => send our precommit vote for `id(v)` and transition to `PRECOMMIT` state.
    - `upon` rule 44: +2/3 of prevotes on nil => send our precommit vote for nil and transition to `PRECOMMIT` state.
  - Sending precommit vote is carried by either a `upon` rule (line 44) or above timeout.
- Enter `PRECOMMIT`: the process enters to this state listens for +2/3 precommits