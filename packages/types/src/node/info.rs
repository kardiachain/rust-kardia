use crate::node;
use core::fmt::{self, Display};

pub struct Info {
    /// Node Id
    pub id: node::Id,

    /// Listen address
    pub listen_addr: ListenAddress,

    /// Moniker
    /// TODO: change into Moniker struct?
    pub moniker: String,

}

pub struct ProtocolVersionInfo {
    // P2P protocol version
    pub p2p: u64,

    // Block version
    pub block: u64,

}

/// Listen address information
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListenAddress(String);

impl ListenAddress {
    // Construct `ListenAddress`
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
