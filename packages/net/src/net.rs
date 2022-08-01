use types::node::{self, info::ListenAddress};

/// URI prefix for TCP connections
pub const TCP_PREFIX: &str = "tcp://";

/// Remote address (TCP)
///
/// For TCP-based addresses, this supports both IPv4 and IPv6 addresses and
/// hostnames.
///
/// If the scheme is not supplied (i.e. `tcp://`) when parsing
/// from a string, it is assumed to be a TCP address.
pub enum Address {
    /// TCP connections
    Tcp {
        /// Remote peer ID
        peer_id: Option<node::Id>,

        /// Hostname or IP address
        host: String,

        /// Port
        port: u16,
    },
}

impl Address {
    /// Convert `ListenAddress` to a `net::Address`
    pub fn from_listen_address(address: &ListenAddress) -> Option<Self> {
        let raw_address = address.as_str();
        if raw_address.starts_with(TCP_PREFIX) {
            raw_address.parse().ok();
        } else {
            
        }
    }
}
