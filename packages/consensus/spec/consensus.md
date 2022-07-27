# Kardia consensus algorithm
This consensus algorithm is based on DPoS-BFT.

## Terminologies
- The network is composed of optionally connected _nodes_. Nodes
  directly connected to a particular node are called _peers_.
- The consensus process in deciding the next block (at some _height_
  `H`) is composed of one or many _rounds_.
- `Propose`, `Prevote`, and `Precommit`
  represent state machine states of a round. (aka `RoundStep` or
  just "step").
- A node is said to be _at_ a given height, round, and step, or at
  `(H,R,S)`, or at `(H,R)` in short to omit the step.
- `v`: value, aka. block; `id(v)`: hash of block `v`; `h_p`: height of process
- `upon` rule is triggered once the condition is satisfied. 
  - The condition `+2/3 <PRECOMMIT, h_p, r, id(v)>` is evaluated to `true` once there is two third of majority `PRECOMMIT` on block `v` at height `h_p` and round `r`.
  - Some of the rules ends with ”for the ﬁrst time” constraint to denote that it is triggered only the ﬁrst time a corresponding condition evaluates to true.
- The variables with index `p` are process local state variables, while variables without index p are value placeholders.
- Algorithm proceeds in rounds, where each round has a dedicated *proposer*. Function `proposer(h, r)` returns the proposer for the round `r` at height `h`.
- There are three timouts: `timeoutPropose`, `timeoutPrevote` and `timeoutPrecommit`. The timeouts prevent the algorithm from blocking (waiting for some condition to be true). Timeouts are increased every new round `r`: `timeoutX(r) = initTimeoutX + r*timeoutDelta` where `X` could be `Propose`, `Prevote` or `Precommit`, they are reset for every new height.
- `h_p` and `round_p` are attached to every message. Finally, a process also stores an array of decisions, `decision_p` (assumes a sequence of consensus instances, one for each height).
- `PROPOSAL` message that carries value `v`, `PREVOTE` and `PRECOMMIT` messages carry value id `id(v)`.
- A validator sends `PREVOTE(id(v))` when it evaluates `PROPOSAL(v)` is valid, otherwise `PREVOTE(nil)`.
- A validator sends `PRECOMMIT(id(v))` when it receives +2/3 `PREVOTE(id(v))`, otherwise `PRECOMMIT(nil)`.
- A validator proceeds to `COMMIT` when it receives +2/3 `PRECOMMIT(id(v))`
- During the prevote step, a validator "locks" value `v` iff it has +2/3 `PREVOTE` (or before sending `PRECOMMIT(id(v))`). If there is another proposal has +2/3 prevotes, the process moves to lock that new proposal. The lock is reset every new height. `lockedValue` is used to store forementioned value and `lockedRound` is the round `r` when `validValue` gets assigned. Locking mechanism prevents proposing 2 different proposals for the same height. (upon rule 5)
- Whenever the consensus receives +2/3 prevotes of a proposal for the first time, the process considers that is a valid proposal. `validValue` is used to store forementioned value and `validRound` is the round `r` when `validValue` gets assigned. This is considered as a valid proposal caching method which helps to reduce the time to propose a new proposal.

## Pseudocode
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
- Enter `PRECOMMIT`: the process enters to this state listens for +2/3 precommits of `id(v)` to commit `v` (TODO: finalizing commit).
  - A precommit timeout is scheduled in `upon` rule 7 with `height` and `round`, function `OnTimeoutPrecommit(hieght, round)` will be executed when timeout.
    - `upon` rule 7: +2/3 of any precommits received => schedule timeout precommit.
  - The execution of `StartRound()` (which enter `PROPOSE` state) is guaranteed by either `upon` rule 8 rightaway or after above timeout.
- The `upon` rule 9 helps it catching up the latest round of other processes.

## Auxiliary processes
These auxiliary processes are in charged to:
- decide/validate proposal
  - [Deciding proposal](./proposal.md#deciding-proposal)
  - [Validating proposal](./proposal.md#validating-proposal)
- process incoming messages (proposal or votes) and check `upon` rules
  - [Processing messages](./messages.md#processing-messages)
- commit, apply new block
  - [Commit specification](./commit.md#commit-specification)
- gossip protocol
  - [Gossip protocol specification](./gossip.md#gossip-protocol-specification)