use thiserror::Error;
use super::peer::ChannelId;

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
    #[error("decode proto error")]
    DecodeProtoError,
    #[error("encode proto error")]
    EncodeProtoError,
    #[error("invalid step")]
    ErrInvalidStep,
}