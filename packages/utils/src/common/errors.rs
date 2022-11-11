use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("{err:?}")]
    MsgError{
        err: String
    },

    #[error("{err:?}")]
    AddrError{
        err: String,
        addr: String,
    },
}