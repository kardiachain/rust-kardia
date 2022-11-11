//! Channels (RPC types)

mod id;

use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};

pub use self::id::Id;

/// Channels
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
    /// Channel ID
    pub id: Id,

    /// Capacity of the send queue
    pub send_queue_capacity: u64,

    /// Size of the send queue
    pub send_queue_size: u64,

    /// Priority value
    pub priority: u64,

    /// Amount of data recently sent
    pub recently_sent: u64,
}

/// Channel collections
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Default)]
pub struct Channels(String);

impl Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
