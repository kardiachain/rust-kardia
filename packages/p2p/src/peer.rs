use std::str::Bytes;
use crate::key::Id;
use crate::node_info::NodeInfo;

use crate::conn::mconnection::MConnection;
// peer implements Peer.
//
// Before using a peer, you will need to perform a handshake on connection.
// go-kardia/p2p/peer.go#L100
struct Peer {
    // raw peerConn and the multiplex connection
    peer_conn: PeerConn,
    mconn: MConnection,

    // peer's node info and the channel it knows about
	// channels = nodeInfo.Channels
	// cached to avoid copying nodeInfo in hasChannel
    node_info: Box<dyn NodeInfo>,
    channels: Vec<u8>,
}

// peerConn contains the raw connection and its config.
pub struct PeerConn {
    pub outbound: bool,
}

impl Peer {

    pub fn string(&self) -> String {
        if self.peer_conn.outbound {
            return "".to_string()
        }
        "".to_string()

    }

    pub fn set_logger() {}

    pub fn on_start() {}
    
    pub fn flush_stop() {}

    pub fn on_stop() {}

    pub fn id(&self) -> Id {
        self.node_info.id()
    }

    pub fn is_outbound(&self) -> bool {
        self.peer_conn.outbound
    }

    pub fn is_persistent() -> bool {
        false
    }

    pub fn node_info(&self) -> &Box<dyn NodeInfo> {
        &self.node_info
    }

    pub fn send(ch_id : Bytes, msg: Vec<Bytes>) -> bool {
        false
    }
}