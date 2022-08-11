use std::fmt::{self};
use std::net::{TcpStream, IpAddr};

use crate::key::Id;
use crate::netaddress::NetAddress;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum P2PError {
    // ErrFilterTimeout indicates that a filter operation timed out.
    #[error("filter timed out")]
    ErrFilterTimeout,

    // ErrSwitchDuplicatePeerID to be raised when a peer is connecting with a known
    // ID.
    #[error("duplicate peer ID {id:?}")]
    ErrSwitchDuplicatePeerID{
        id: Id,
    },

    // ErrSwitchDuplicatePeerIP to be raised whena a peer is connecting with a known
    // IP.
    #[error("duplicate peer IP {ip:?}")]
    ErrSwitchDuplicatePeerIP{
        ip: IpAddr,
    },

    // ErrSwitchConnectToSelf to be raised when trying to connect to itself.
    #[error("connect to self: {addr:?}")]
    ErrSwitchConnectToSelf{
        addr: NetAddress
    },

    #[error("failed to authenticate peer. Dialed {dialed:?}, but got peer with ID {got:?}")]
    ErrSwitchAuthenticationFailure {
        dialed: NetAddress,
        got: Id,
    },

    // ErrTransportClosed is raised when the Transport has been closed.
    #[error("transport has been closed")]
    TransportClosed,

    #[error("ErrNetaddress {addr:?} does not contain ID")]
    ErrNetAddressNoId{addr: String},

    #[error("invalid address {addr:?}: {err:?}")]
    ErrNetAddressInvalid{
        addr: String,
        err : String,
    },

    #[error("error looking up host {addr:?}: {err:?}")]
    ErrNetAddressLookup{
        addr: String,
        err : String,
    },

    // ErrCurrentlyDialingOrExistingAddress indicates that we're currently
    // dialing this address or it belongs to an existing peer.
    #[error("connection with {addr:?} has been established or dialed")]
    ErrCurrentlyDialingOrExistingAddress{addr: String}
}

#[derive(Debug)]
pub struct ErrRejected {
    addr : NetAddress,
    conn : Option<TcpStream>,
    err : String,
    id : Id,
    is_auth_failure : bool,
    is_duplicate: bool,
    is_filtered: bool,
    is_incompatible: bool,
    is_node_info_invalid: bool,
    is_self: bool
}

impl ErrRejected {
    pub fn addr(&self) -> NetAddress {
        self.addr
    }
}

impl fmt::Display for ErrRejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "SuperError is here!")

        if self.is_auth_failure {
            return write!(f, "auth failure: {}", self.err)
        }

        if self.is_duplicate {
            if self.conn.is_some() {
                return write!(f, "duplicate CONN<{:?}>", self.conn.unwrap())
            }

            if self.id != "" {
                return write!(f, "duplicate ID<{}>", self.id)
            }
        }

        if self.is_filtered {
            if self.conn.is_some(){
                return write!(f, "filtered CONN<{:?}>", self.conn.unwrap())
            }
            if self.id != "" {
                return write!(f, "filtered ID<{}>", self.id)
            }
        }

        if self.is_incompatible {
            return write!(f, "incompatible: {}>", self.err)
        }

        if self.is_node_info_invalid {
            return write!(f, "invalid NodeInfo: {}", self.err)
        }

        if self.is_self {
            return write!(f, "self ID<{}>", self.id)
        }

        write!(f, "{}", self.err)
    }
}

impl ErrRejected {

    // IsAuthFailure when Peer authentication was unsuccessful.
    pub fn is_auth_failure(&self) -> bool {
        self.is_auth_failure
    }

    // IsDuplicate when Peer ID or IP are present already.
    pub fn is_duplicate(&self) -> bool {
        self.is_duplicate
    }

    // IsFiltered when Peer ID or IP was filtered.
    pub fn is_filtered(&self) -> bool {
        self.is_filtered
    }

    // IsIncompatible when Peer NodeInfo is not compatible with our own.
    pub fn is_incompatible(&self) -> bool {
        self.is_incompatible
    }

    // IsNodeInfoInvalid when the sent NodeInfo is not valid.
    pub fn is_node_info_invalid(&self) -> bool {
        self.is_node_info_invalid
    }

    // IsSelf when Peer is our own node.
    pub fn is_self(&self) -> bool {
        self.is_self
    }
}
