# consensus package
This package implements the DPoS-BFT consensus engine. It maintains internal state and peer state via transporting messages via `kardia-sentry` service.
Its main components are:
- ticker for processing data in round and steps
- consensus reactor: handle messages from peer or internal state changes via channels (state, data, vote, votebit)
- models: state, peer state, message


## Models
```rust
struct TimeoutInfo {
    duration
    height uint64
    round uint32
    step: RoundStepType
}
```

## Message types

## Ticker
TimeoutTicker is a timer that schedules timeouts conditional on the height/round/step in the timeoutInfo.
`TimeoutTicker` trait: 
```rust
trait TimeoutTicker {
    pub fn start() -> Result<()>;
    pub fn stop() -> ();
    pub fn is_running -> Result<bool>;
    pub fn schedule_timeout(ti: TimeoutInfo) -> Result<()>;
    fn timeout_routine() -> ();
}
```
It has 3 important properties:
- timer: a thread that sleeps in given duration, when it awakes after sleeping, it will send a message on `timer.C` channel
- tickChan: for scheduling timouts
- tockChan: for notifying about them
### `start()`
It starts the timeout routine.
### `timeout_routine()`
It's an infinite loop (terminatable by listening on a context for graceful shutdown). In particular:
```
Loop:
    Listening on tickChan to start a new timer:
        - If old height/round/step, then skip
        - Stop the last timer
        - Update to new timeout info and reset timer
    Listening on timer.C to send timout info to tockChan
```
### `schedule_timeout()`
It schedules a new timeout by sending on the internal tickChan.
```go
func (t *timeoutTicker) ScheduleTimeout(ti timeoutInfo) {
	t.tickChan <- ti
}
```
##

## Consensus reactor
Methods:
```
OnStart()
OnStop()
SwitchToConsensus()
subscribeToBroadcastEvents()
updateRoundStateRoutine()
gossipDataRoutine()
gossipVotesRoutine()
queryMaj23Routine()
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
### processPeerUpdate()
processPeerUpdate process a peer update message. For new or reconnected peers,
we create a peer state if one does not exist for the peer, which should always
be the case, and we spawn all the relevant goroutine to broadcast messages to
the peer. During peer removal, we remove the peer for our set of peers and
signal to all spawned goroutines to gracefully exit in a non-blocking manner.

### OnStop()
OnStop stops the reactor by signaling to all spawned goroutines to exit and blocking until they all exit, as well as unsubscribing from events and stopping state.

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
OnStart loads the latest state via the WAL, and starts the timeout and receive routines. If peerID == "", the msg is considered internal. Messages are added to the appropriate queue (peer or internal). If the queue is full, the function may block.

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
It checks `timeout.Step`, in case the step is:
- *RoundStepNewHeight*: `cs.enterNewRound()`,
- *RoundStepNewRound*: `cs.enterPropose()`,
- *RoundStepPropose*: `cs.enterPrevote()`,
- *RoundStepPrevoteWait*: `cs.enterPrecommit()`,
- *RoundStepPrecommitWait*: `cs.enterPrecommit()` `cs.enterNewRound()`.