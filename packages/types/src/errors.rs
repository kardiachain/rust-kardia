use thiserror::Error;

use crate::{evidence::DuplicateVoteEvidence, vote::Vote};

#[derive(Error, Debug)]
pub enum AddVoteError {
    #[error("unexpect mismatch: got `{0}`/`{1}`/`{2}`, expected `{3}`/`{4}`/`{5}`")]
    UnexpectedMismatch(u64, u32, &'static str, u64, u32, &'static str),
    #[error("cannot find validator in validator set: validator_address=`{0}`")]
    ValidatorNotFound(String),
    #[error("same vote but wrong signature, existing={existing:?} new_vote={new_vote:?}")]
    NonDeterministicSignature { existing: Vote, new_vote: Vote },
    #[error("invalid vote error: {0:?}")]
    InvalidVote(VoteError),
    #[error("conflicting votes: evidence={0:?}")]
    NewConflictingVoteError(DuplicateVoteEvidence),
    #[error("vote from unwanted round")]
    VoteFromUnwantedRound,
    #[error("conflicting vote: {0:?}")]
    ConflictingVote(Vote)
}

#[derive(Error, Debug)]
pub enum VoteError {
    #[error("invalid signature")]
    InvalidSignature,
    #[error("failed to create vote sign bytes")]
    CreateVoteSignBytesError,
}
