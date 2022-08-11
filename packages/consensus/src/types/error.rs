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
        write!(f, "add peer error: try_lock failed")
    }
}

impl Error for UnknownChannelIdError {}