- [Vote specification](#vote-specification)
  - [Processing vote message](#processing-vote-message)
      - [Vote validations](#vote-validations)
      - [Adding vote process](#adding-vote-process)
      - [Check for upon rules](#check-for-upon-rules)
      - [Evidence](#evidence)

# Vote specification
## Processing vote message
Vote messages are added into vote set. They are checked for validity, an evidence will be thrown for peer violation. 

The consensus stores only votes of `height_p` in `Votes: HeightVoteSet`. Precommit votes from previous height is stored in `LastCommit: VoteSet`. 

Vote messages are grouped by round `r` and vote type in `HeightVoteSet` as following:
```
// all votes of a height
struct HeightVoteSet {
  height
  valSet // validator set

  round // max tracked round
  roundVoteSets // a map between round and round's vote set
  peerCatchupRounds // TODO: what is it?
}
 
// all votes of a round
struct RoundVoteSet {
  Prevotes: VoteSet // all prevotes of round
  Precommits: VoteSet // all precommits of round
}

// 
struct VoteSet {

}
```
#### Vote validations
- Validate vote: Ensure height, round, step are match; ensure signer, validator...

#### Adding vote process
- Add vote by height, round, block id
- Check vote conflict, throw evidence to punish peer.

Special cases, must be checked before normal case is run:
- Late precommit from last height (`height_p - 1`) should be add in `LastCommit`.
#### Check for upon rules
For every added vote, the process performs the check for `upon` rules and triggers rule's execution as soon as the rule is satisfied.

#### Evidence
