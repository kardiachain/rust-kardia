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
updateRoundStateRoutine()
gossipDataRoutine()
gossipVotesRoutine()
queryMaj23Routine()
```

## Consensus state
Methods:
```
receiveRoutine()
```

### receiveRoutine()
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