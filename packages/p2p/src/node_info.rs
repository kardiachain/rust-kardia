use crate::{key::Id, netaddress::NetAddress};

const MAX_NODE_INFO_SIZE : i32 = 10248; // 10KB
const MAX_NUM_CHANNELS : i32 = 16; // plenty of room for upgrades, for now

pub fn max_node_info_size() -> i32 {
    return MAX_NODE_INFO_SIZE
}

pub trait NodeInfo: NodeInfoAddress + NodeInfoTransport {
    // Id
    fn id(&self) -> Id;
}

pub trait NodeInfoAddress {
    fn net_address(&self);
}

pub trait NodeInfoTransport {
    fn validate(&self);
    fn compatible_with(&self, other: Box<dyn NodeInfo>);
}


// ProtocolVersion contains the protocol versions for the software.
pub struct ProtocolVersion {
    p2p : u64,
    block : u64,
    app : u64
}

const DEFAULT_PROTOCOL_VERSION : ProtocolVersion = ProtocolVersion{p2p: 1, block: 1, app: 0};

pub fn new_protocol_version(p2p : u64, block : u64, app : u64) -> ProtocolVersion {
    ProtocolVersion { p2p: p2p, block: block, app: app }
}

// ------------------

// DefaultNodeInfo is the basic node information exchanged
// between two peers during the P2P handshake.
pub struct DefaultNodeInfo {
    pub protocol_version : ProtocolVersion,

    // Authenticate
    // TODO: replace with NetAddress
    pub default_node_id : Id,
    pub listen_addr : String,

    // Check compatibility
    // Channels are HexBytes so easier to read as JSON
    pub network: String,
    pub version: String,
    pub channels: Vec<u8>,  // TODO: should change into bytes lib?

    // ASCIIText fields
    pub moniker: String,
    pub other: DefaultNodeInfoOther
}

// DefaultNodeInfoOther is the misc. applcation specific data
pub struct DefaultNodeInfoOther{
    pub tx_index: String,
    pub rpc_address: String
}

impl DefaultNodeInfo { 
    // ID returns the node's peer ID.
    pub fn id(&self) -> Id {
        self.default_node_id.clone()
    }

    // Validate checks the self-reported DefaultNodeInfo is safe.
    // It returns an error if there
    // are too many Channels, if there are any duplicate Channels,
    // if the ListenAddr is malformed, or if the ListenAddr is a host name
    // that can not be resolved to some IP.
    // TODO: constraints for Moniker/Other? Or is that for the UI ?
    // JAE: It needs to be done on the client, but to prevent ambiguous
    // unicode characters, maybe it's worth sanitizing it here.
    // In the future we might want to validate these, once we have a
    // name-resolution system up.
    // International clients could then use punycode (or we could use
    // url-encoding), and we just need to be careful with how we handle that in our
    // clients. (e.g. off by default).
    pub fn validate(&self) -> Option<()> {
        todo!()
    }

    // CompatibleWith checks if two DefaultNodeInfo are compatible with eachother.
    // CONTRACT: two nodes are compatible if the Block version and network match
    // and they have at least one channel in common.
    pub fn compatible_with(&self, other_info: impl NodeInfo) -> Option<()> {
        todo!()
    }

    pub fn net_address(&self) -> Option<&NetAddress> {
        todo!()
    }

    pub fn to_proto(&self) {
        todo!()
    }
}

pub fn default_node_info_from_to_proto() -> DefaultNodeInfo {
    todo!()
}