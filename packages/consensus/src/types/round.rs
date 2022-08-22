extern crate num;
extern crate num_derive;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, PartialEq, PartialOrd, FromPrimitive)]
pub enum CanonicalRoundStep {
    NewHeight = 1,
    NewRound = 2,
    Propose = 3,
    Prevote = 4,
    PrevoteWait = 5,
    Precommit = 6,
    PrecommitWait = 7,
    Commit = 8,
}

impl From<CanonicalRoundStep> for RoundStep {
    fn from(crs: CanonicalRoundStep) -> Self {
        match crs {
            CanonicalRoundStep::NewHeight
            | CanonicalRoundStep::NewRound
            | CanonicalRoundStep::Propose => Self::Propose,

            CanonicalRoundStep::Prevote | CanonicalRoundStep::PrevoteWait => Self::Prevote,

            CanonicalRoundStep::Precommit
            | CanonicalRoundStep::PrecommitWait
            | CanonicalRoundStep::Commit => Self::Precommit,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, FromPrimitive)]
pub enum RoundStep {
    Unknown = 0,
    Propose = 11,
    Prevote = 12,
    Precommit = 13,
}

impl RoundStep {
    pub fn is_valid(self) -> bool {
        return self != Self::Unknown
    }
}