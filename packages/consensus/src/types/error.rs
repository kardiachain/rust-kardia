use kai_types::misc::ChannelId;
use thiserror::Error;

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