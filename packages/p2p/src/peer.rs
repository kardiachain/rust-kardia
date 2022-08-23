use crate::key::Id;
use crate::netaddress::NetAddress;
use crate::node_info::{NodeInfo, NodeInfoAddress};
use std::net::TcpStream;

use crate::conn::mconnection::{MConnection, ConnectionStatus};

pub trait PeerTrait {
    // service


    fn flush_stop();

    fn id(&self) -> Id;            // peer's cryptographic ID
    fn remote_ip();     // ?? remote IP of the connection
    fn remote_addr();   // remote address of the connection

    fn is_outbound(&self) -> bool;             // did we dial the peer
    fn is_persistent(&self) -> bool;           // do we redial this peer when we disconnect

    fn close_conn();    // close original connection
    fn node_info(&self) -> Box<dyn NodeInfo>;  // peer's info
    fn status() -> ConnectionStatus;
    fn socket_addr(&self) -> &NetAddress;     // actual address of the socket

    fn send() -> bool;
    fn try_send() -> bool;

    fn set();
    fn get();
    fn has_channel(&self, ch_id: u8) -> bool;

}

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
struct PeerConn {
    outbound: bool,
    persistent : bool,
    conn : TcpStream,
    socket_address: &'static NetAddress,

}

impl PeerTrait for Peer {
    fn flush_stop() {
        todo!()
    }

    fn id(&self) -> Id {
        self.node_info.id()
    }

    fn remote_ip() {
        todo!()
    }

    fn remote_addr() {
        todo!()
    }

    fn is_outbound(&self) -> bool {
        self.peer_conn.outbound
    }

    fn is_persistent(&self) -> bool {
        self.peer_conn.persistent
    }

    fn close_conn() {
        todo!()
    }

    // NodeInfo returns a copy of the peer's NodeInfo.
    fn node_info(&self) -> Box<dyn NodeInfo> {
        todo!()
    }

    fn status() -> ConnectionStatus {
        todo!()
    }

    // SocketAddr returns the address of the socket.
    // For outbound peers, it's the address dialed (after DNS resolution).
    // For inbound peers, it's the address returned by the underlying connection
    // (not what's reported in the peer's NodeInfo).
    fn socket_addr(&self) -> &NetAddress {
        self.peer_conn.socket_address
    }

    fn send() -> bool {
        todo!()
    }

    fn try_send() -> bool {
        todo!()
    }

    fn set() {
        todo!()
    }

    fn get() {
        todo!()
    }

    fn has_channel(&self, ch_id: u8) -> bool {
        todo!()
    }
}

// -----------------------------------
// helper funcs

fn create_mconn() {
    todo!()
}