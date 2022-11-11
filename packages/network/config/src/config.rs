use serde::{Deserialize, Serialize};

/// Parse `config.yaml` file
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    #[serde(rename = "Node")]
    pub node: NodeConfig,
    #[serde(rename = "MainChain")]
    pub mainchain: MainChainConfig,
    #[serde(rename = "GenTxs")]
    pub gen_txs: GenTxsConfig,
    #[serde(rename = "Debug")]
    pub debug: DebugConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct NodeConfig {
    #[serde(rename = "Name")]
    name: String,
    #[serde(skip)]
    version: String,
    #[serde(rename = "DataDir")]
    data_dir: String,
    #[serde(rename = "HTTPHost")]
    pub http_host: String,
    #[serde(rename = "HTTPPort")]
    pub http_port: String,
    #[serde(rename = "HTTPModules")]
    http_modules: Vec<String>,
    #[serde(rename = "HTTPVirtualHosts")]
    http_virtual_hosts: Vec<String>,
    #[serde(rename = "HTTPCors")]
    http_cors: Vec<String>,
    #[serde(rename = "WSHost")]
    ws_host: String,
    #[serde(rename = "WSPort")]
    ws_port: String,
    /// cors, use "*" to accept all
    #[serde(rename = "WSOrigins")]
    ws_origins: Vec<String>,
    #[serde(rename = "P2P")]
    pub p2p: P2PConfig,
    /// crit, error, warn, info, debug, trace
    #[serde(rename = "LogLevel")]
    log_level: String,
    #[serde(rename = "FastSync")]
    fast_sync: FastSyncConfig,
    #[serde(rename = "TimeOutForStaticCall")]
    timeout_for_static_call: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct P2PConfig {
    #[serde(rename = "PrivateKey")]
    pub private_key: String,
    #[serde(rename = "ListenAddress")]
    listen_address: String,
    #[serde(rename = "InboundPeers")]
    inbound_peers: i32,
    #[serde(rename = "OutboundPeers")]
    outbound_peers: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FastSyncConfig {
    /// true if this node allow and be able to fastsync, otherwise false
    #[serde(rename = "Enable")]
    enable: bool,
    /// maximum peer is allowed to receive fastsync blocks from this node at a time. Type int
    #[serde(rename = "MaxPeers")]
    max_peers: i32,
    /// maximum number of blocks in a batch sync. Type int
    #[serde(rename = "TargetPending")]
    target_pending: i32,
    /// maximum response time from a peer in second. Type int
    #[serde(rename = "PeerTimeout")]
    peer_timeout: i32,
    /// minimum receive rate from peer, otherwise prune. Type int64
    #[serde(rename = "MinRecvRate")]
    min_recv_rate: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MainChainConfig {
    #[serde(rename = "ServiceName")]
    service_name: String,
    #[serde(rename = "ChainId")]
    chain_id: String,
    #[serde(rename = "NetworkId")]
    network_id: String, 
    /// accept tx sync process or not (1 is yes, 0 is no)
    #[serde(rename = "AcceptTxs")]
    accept_txs: String,
    #[serde(rename = "Seeds")]
    seeds: Vec<String>,
    
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DatabaseConfig {
    /// directory stores leveldb
    #[serde(rename = "Dir")]
    dir: String,
    /// cache is used in leveldb
    #[serde(rename = "Cache")]
    cache: i32,
    /// handles is used in leveldb
    #[serde(rename = "Handles")]
    handles: i32,
    /// Specify whether drop database or not (0 is no, 1 is yes)
    #[serde(rename = "Drop")]
    drop: i32,
}

/// Devnet config to dynamize txs processing
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GenTxsConfig {
    #[serde(rename = "Type")]
    txs_type: i32,
    #[serde(rename = "NumTxs")]
    num_txs: i32,
    #[serde(rename = "Delay")]
    delay: i32,
    #[serde(rename = "Index")]
    index: i32,
}

/// Devnet config for debugging purposes
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DebugConfig {
    #[serde(rename = "Port")]
    port: String,
}
