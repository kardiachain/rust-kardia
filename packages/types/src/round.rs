extern crate num;
extern crate num_derive;
use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, FromPrimitive)]
pub enum RoundStep {
    Unknown = 0,

    CanonicalNewHeight = 1,
    CanonicalNewRound = 2,
    CanonicalPropose = 3,
    CanonicalPrevote = 4,
    CanonicalPrevoteWait = 5,
    CanonicalPrecommit = 6,
    CanonicalPrecommitWait = 7,
    CanonicalCommit = 8,

    Propose = 11,
    Prevote = 12,
    Precommit = 13,
}

impl RoundStep {
    pub fn is_valid(self) -> bool {
        return self != Self::Unknown
    }
}