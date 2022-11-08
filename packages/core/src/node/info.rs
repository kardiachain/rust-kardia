use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};


/// Node information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Info {
    /// Protocol version information
    pub protocol_version: ProtocolVersionInfo,

    /// Node ID
    pub id: crate::node::id::Id,

    /// Listen address
    pub listen_addr: ListenAddress,

    // /// Tendermint network / chain ID,
    // pub network: chain::Id,

    // /// Tendermint version
    // pub version: Version,

    // /// Channels
    // pub channels: Channels,

    // /// Moniker
    // pub moniker: Moniker,

    // /// Other status information
    // pub other: OtherInfo,
}

/// Protocol version information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProtocolVersionInfo {
    /// P2P protocol version
    pub p2p: u64,

    /// Block version
    pub block: u64,

    /// App version
    pub app: u64,
}

/// Listen address information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListenAddress(String);

impl ListenAddress {
    /// Construct `ListenAddress`
    pub fn new(s: String) -> ListenAddress {
        ListenAddress(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ListenAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
