pub enum CanonicalRoundStep {
    RoundStepNewHeight = 1,
    RoundStepNewRound = 2,
    RoundStepPropose = 3,
    RoundStepPrevote = 4,
    RoundStepPrevoteWait = 5,
    RoundStepPrecommit = 6,
    RoundStepPrecommitWait = 7,
    RoundStepCommit = 8,
}

pub enum RoundStep {
    RoundStepPropose,
    RoundStepPrevote,
    RoundStepPrecommit,
}