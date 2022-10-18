use std::time::{Duration, Instant};
use libp2p::{
    ping::{ Ping, PingConfig}, identify::{Identify, IdentifyConfig, IdentifyInfo}, core::ConnectedPoint, 
    core::{PublicKey}, PeerId
};
use smallvec::SmallVec;
use fnv::FnvHashMap;
use log::{debug, error, trace};

/// Time after we disconnect from a node before we purge its information from the cache.
const CACHE_EXPIRE: Duration = Duration::from_secs(10 * 60);
/// Interval at which we perform garbage collection on the node info.
const GARBAGE_COLLECT_INTERVAL: Duration = Duration::from_secs(2 * 60);

/// Implementation of `NetworkBehaviour` that holds information about peers in cache.
pub struct PeerInfoBehaviour {
	/// Periodically ping nodes, and close the connection if it's unresponsive.    
    ping: Ping,
	/// Periodically identifies the remote and responds to incoming requests.
    identify: Identify,
    /// Information that we know about all nodes.
    nodes_info: FnvHashMap<PeerId, NodeInfo>,
}

/// Information about a node we're connected to.
#[derive(Debug)]
struct NodeInfo {
	/// When we will remove the entry about this node from the list, or `None` if we're connected
	/// to the node.
	info_expire: Option<Instant>,
	/// Non-empty list of connected endpoints, one per connection.
	endpoints: SmallVec<[ConnectedPoint; crate::MAX_CONNECTIONS_PER_PEER]>,
	/// Version reported by the remote, or `None` if unknown.
	client_version: Option<String>,
	/// Latest ping time with this node.
	latest_ping: Option<Duration>,
}

impl NodeInfo {
    fn new(endpoint: ConnectedPoint) -> Self {
        let mut endpoints = SmallVec::new();
        endpoints.push(endpoint);
        Self { info_expire: None, endpoints, client_version: None, latest_ping: None }
    }
}

impl PeerInfoBehaviour {
    /// Builds a new PeerInfoBehaviour
    pub fn new(user_agent: String, local_public_key: PublicKey) -> Self {
        let identify = {
			let cfg = IdentifyConfig::new("/substrate/1.0".to_string(), local_public_key)
				.with_agent_version(user_agent);
			Identify::new(cfg)
		};

		Self {
			ping: Ping::new(PingConfig::new()),
			identify,
			nodes_info: FnvHashMap::default(),
		}
    }

    /// Borrows `self` and returns a struct giving access to the information about a node.
	///
	/// Returns `None` if we don't know anything about this node. Always returns `Some` for nodes
	/// we're connected to, meaning that if `None` is returned then we're not connected to that
	/// node.
	pub fn node(&self, peer_id: &PeerId) -> Option<Node> {
		self.nodes_info.get(peer_id).map(Node)
	}

    /// Inserts a ping time in the cache. Has no effect if we don't have any entry for that node,
	/// which shouldn't happen.
	fn handle_ping_report(&mut self, peer_id: &PeerId, ping_time: Duration) {
		trace!(target: "sub-libp2p", "Ping time with {:?}: {:?}", peer_id, ping_time);
		if let Some(entry) = self.nodes_info.get_mut(peer_id) {
			entry.latest_ping = Some(ping_time);
		} else {
			error!(target: "sub-libp2p",
				"Received ping from node we're not connected to {:?}", peer_id);
		}
	}

	/// Inserts an identify record in the cache. Has no effect if we don't have any entry for that
	/// node, which shouldn't happen.
	fn handle_identify_report(&mut self, peer_id: &PeerId, info: &IdentifyInfo) {
		trace!(target: "sub-libp2p", "Identified {:?} => {:?}", peer_id, info);
		if let Some(entry) = self.nodes_info.get_mut(peer_id) {
			entry.client_version = Some(info.agent_version.clone());
		} else {
			error!(target: "sub-libp2p",
				"Received pong from node we're not connected to {:?}", peer_id);
		}
	}
}

/// Gives access to the information about a node.
pub struct Node<'a>(&'a NodeInfo);

impl<'a> Node<'a> {
	/// Returns the endpoint of an established connection to the peer.
	///
	/// Returns `None` if we are disconnected from the node.
	pub fn endpoint(&self) -> Option<&'a ConnectedPoint> {
		self.0.endpoints.get(0)
	}

	/// Returns the latest version information we know of.
	pub fn client_version(&self) -> Option<&'a str> {
		self.0.client_version.as_deref()
	}

	/// Returns the latest ping time we know of for this node. `None` if we never successfully
	/// pinged this node.
	pub fn latest_ping(&self) -> Option<Duration> {
		self.0.latest_ping
	}
}