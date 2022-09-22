use kai_types::misc::ChannelId;
use thiserror::Error;

use super::messages::VoteMessage;

#[derive(Error, Debug)]
pub enum ConsensusReactorError {
    #[error("add peer error: `{0}`")]
    AddPeerError(String),
    #[error("lock failed: `{0}`")]
    LockFailed(String),
    #[error("invalid channel id: `{0}`")]
    UnknownChannelIdError(ChannelId),
    #[error("unknown message type")]
    UnknownMessageTypeError,
    #[error("unexpected message type: {0}")]
    UnexpectedMessageTypeError(String),
    #[error("decode proto error")]
    DecodeProtoError,
    #[error("encode proto error")]
    EncodeProtoError,
    #[error("invalid step")]
    ErrInvalidStep,
}

#[derive(Error, Debug)]
pub enum ConsensusStateError {
    #[error("add vote error")]
    AddingVote,
    #[error("unknown msg type")]
    UnknownMessageTypeError,
    #[error("lock failed: `{0}`")]
    LockFailed(String),
    #[error("create signed vote error: `{0}`")]
    CreateSignedVoteError(String),
    #[error("verify signature error: `{0}`")]
    VerifySignatureError(String),
    #[error("add block part error: `{0}`")]
    AddBlockPartError(String),
}
