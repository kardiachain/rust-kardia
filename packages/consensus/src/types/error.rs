use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug, Clone)]
pub struct AddPeerError;
impl Display for AddPeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "add peer error: try_lock failed")
    }
}
impl Error for AddPeerError {}

#[derive(Debug, Clone)]
pub struct UnknownChannelIdError;
impl Display for UnknownChannelIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid channel id")
    }
}
impl Error for UnknownChannelIdError {}

#[derive(Debug, Clone)]
pub struct UnknownMessageTypeError;
impl Display for UnknownMessageTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown message type")
    }
}
impl Error for UnknownMessageTypeError {}

#[derive(Debug, Clone)]
pub struct DecodeProtoError;
impl Display for DecodeProtoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "decode proto error")
    }
}
impl Error for DecodeProtoError {}

#[derive(Debug, Clone)]
pub struct EncodeProtoError;
impl Display for EncodeProtoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "encode proto error")
    }
}
impl Error for EncodeProtoError {}