use crate::{netaddress::NetAddress, peer::Peer, conn::{mconnection::{ChannelDescriptor, MConnConfig}, secret_connection::SecretConnection}, base_reactor::Reactor, node_info::{NodeInfo, self}, config::p2p::Node, key::NodeKey};
use core::time;
use std::{error::Error, collections::HashMap, net::{TcpStream, }, time::Duration};
use tokio::net::TcpListener;


const DEFAULT_DIAL_TIMEOUT: time::Duration = Duration::from_secs(1);
const DEFAULT_FILTER_TIMEOUT: time::Duration = Duration::from_secs(5);
const DEFAULT_HANDSHAKE_TIMEOUT: time::Duration = Duration::from_secs(3);


// Transport emits and connects to Peers. The implementation of Peer is left to
// the transport. Each transport is also responsible to filter establishing
// peers specific to its domain.
pub trait Transport {
    fn net_address(&self) -> NetAddress;

    fn accept(&self, peer_cfg: PeerConfig) -> bool;

    fn dial(&self, net_addr: NetAddress, cfg: PeerConfig) -> Result<Peer, Box<dyn Error>>;

    fn cleanup(&self, peer: Peer);
}

// transportLifecycle bundles the methods for callers to control start and stop
// behaviour.
trait TransportLifeCycle {
    fn close() -> Result<(), Box<dyn Error>>;
    fn listen(na: NetAddress) -> Result<(), Box<dyn Error>>;
}

// peerConfig is used to bundle data we need to fully setup a Peer with an
// MConn, provided by the caller of Accept and Dial (currently the Switch). This
// a temporary measure until reactor setup is less dynamic and we introduce the
// concept of PeerBehaviour to communicate about significant Peer lifecycle
// events.
pub struct PeerConfig {
    ch_descs: Vec<ChannelDescriptor>,
    // onPeerError func(Peer, interface{}),
    outbound: bool,
	// isPersistent allows you to set a function, which, given socket address
	// (for outbound peers) OR self-reported address (for inbound peers), tells
	// if the peer is persistent or not.
    is_persistent: fn(&NetAddress)-> bool, 
    reactors_by_ch: HashMap<u8, Box<dyn Reactor>>,
}

// accept is the container to carry the upgraded connection and NodeInfo from an
// asynchronously running routine to the Accept method.
pub struct Accept {
    net_addr: &'static NetAddress,
    conn: TcpStream,
    node_info: Box<dyn NodeInfo>,
    err: Box<dyn Error>
}

pub struct MultiplexTransport {
    na: Option<NetAddress>,
    listener: Option<TcpListener>,
    max_incoming_connections: Option<i32>,

    // acceptc chan accept
	// closec  chan struct{}

	// // Lookup table for duplicate ip and id checks.
	// conns       ConnSet
	// connFilters []ConnFilterFunc

    dial_timeout: time::Duration,
    filter_timeout: time::Duration,
    handshake_timeout: time::Duration,
    node_info: Box<dyn NodeInfo>,
    node_key: NodeKey,

    // TODO(xla): This config is still needed as we parameterise peerConn and
	// peer currently. All relevant configuration should be refactored into options
	// with sane defaults.
    mconfig: MConnConfig
}

impl TransportLifeCycle for MultiplexTransport {
    fn close() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn listen(na: NetAddress) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl MultiplexTransport {
    // NetAddress implements Transport.
    pub fn net_address(&self) -> Option<&NetAddress> {
        self.na.as_ref()
    }

    pub fn accept(&self, cfg: PeerConfig) -> Result<Peer, Box<dyn Error>> {
        todo!()
    }

    // Dial implements Transport.
    pub fn dial(addr: NetAddress, cfg: PeerConfig) -> Result<Peer, Box<dyn Error>> {
        todo!()
    }

    fn accept_peers() {
        todo!()
    }

    // Cleanup removes the given address from the connections set and
    // closes the connection.
    pub fn cleanup(p: Peer) {
        todo!()
    }

    fn filter_conn(c: TcpStream) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn upgrade(c: TcpStream, dialed_addr: &NetAddress) -> Result<(&SecretConnection, Box<dyn NodeInfo>), Box<dyn Error>> {
        todo!()
    }

    fn wrap_peer(c: TcpStream, ni: Box<dyn NodeInfo>, cfg: PeerConfig, socket_addr: &NetAddress) -> Peer {
        todo!()
    }
}

pub fn handshake(c: TcpStream, timeout: time::Duration, node_info: Box<dyn NodeInfo>) -> Result<Box<dyn NodeInfo>, Box<dyn Error>> {
    todo!()
}

pub fn new_multiplex_transport(node_info: Box<dyn NodeInfo>, node_key: NodeKey, m_config: MConnConfig) -> MultiplexTransport{
    MultiplexTransport { na: None, listener: None, max_incoming_connections: None, dial_timeout: DEFAULT_DIAL_TIMEOUT, filter_timeout: DEFAULT_FILTER_TIMEOUT, handshake_timeout: DEFAULT_HANDSHAKE_TIMEOUT, node_info: node_info, node_key: node_key, mconfig: m_config }
}

// MultiplexTransportOption sets an optional parameter on the
// MultiplexTransport.
pub type MultiplexTransportOption = fn(&MultiplexTransport);

// MultiplexTransportFilterTimeout sets the timeout waited for filter calls to
// return.
pub fn multiplex_transport_filter_timeout(time_out: time::Duration) -> MultiplexTransportOption {
    // let set_filter_timeout = |mt: &MultiplexTransport| {
    //     mt.filter_timeout = time_out
    // };
    // return set_filter_timeout;
    todo!()
}