use secp256k1::SecretKey;
use std::{time::Duration};
use kai_utils::crypto::crypto;
use std::error::Error;
use std::path::Path;

const DEFAULT_ADDR_BOOK_NAME : &str = "addrbook.json";

// P2PConfig defines the configuration options for the peer-to-peer networking layer
pub struct P2PConfig {
    // This field must be set to a valid secp256k1 private key.
    pub private_key: Option<SecretKey>,
    pub root_dir: Option<String>,

    // Address to listen for incoming connections
    pub listen_address: String,

    // Address to advertise to peers for them to dial
    pub external_address: String,

    // Comma separated list of seed nodes to connect to
	// We only use these if we can’t connect to peers in the addrbook
	pub seeds: Option<Vec<String>>,

    // Comma separated list of nodes to keep persistent connections to
    pub persistent_peers: Option<String>,

    // UPNP port forwarding
	pub upnp: bool,

    // Path to address book
    pub addr_book: String,

    // Set true for strict address routability rules
	// Set false for private or local networks
    pub addr_book_strict: bool,

    pub max_num_inbound_peers: i32,

    pub max_num_outbound_peers: i32,

    pub unconditional_peer_ids: Option<String>,

    pub persistent_peers_max_dial_period: Duration,

    pub flush_throttle_timeout: Duration,

    pub max_packet_msg_payload_size: i32,

    pub send_rate: i64,

    pub recv_rate: i64,

    pub pex_reactor: bool,

    pub seed_mode: bool,

    pub private_peer_ids: Option<String>,

    pub allow_duplicate_ip: bool,

    pub handshake_timeout: Duration,

    pub dial_timeout: Duration,

    pub test_dial_fail: bool,

    pub test_fuzz: bool,

    // pub test_fuzz_config: &FuzzConnConfig
}

pub struct Config {
    pub node: Node,
    pub main_chain: Chain,

}

pub struct Node {
    pub p2p: P2P,
    
    pub name : String,
    pub data_dir: String,
    pub http_host: String,
    pub http_port: i32,
    pub http_modules: Vec<String>,
    pub http_virtual_hosts: Vec<String>,
    pub http_cors: Vec<String>,
}

pub struct Chain {
    // implement seeds first, will add the rest later
    pub seeds: Vec<String>
}

#[derive(Clone)]
pub struct P2P {
        pub listen_address : String,
        pub private_key : String,
        pub inbound_peers: i32,
        pub outbound_peers: i32
}

impl Config {
    pub fn get_p2p_config(&self) -> Result<P2PConfig, Box<dyn Error>>{
        let priv_key;
        let peer = self.node.p2p.clone();
        if peer.private_key != "" {
            priv_key = crypto::hex_to_ecdsa(peer.private_key)?;
        } else {
            priv_key = crypto::generate_key();
        }
        let mut p2p_config = default_p2p_config();
        p2p_config.private_key = Some(priv_key);
        p2p_config.seeds = Some(self.main_chain.seeds.clone());
        p2p_config.listen_address = self.node.p2p.listen_address.clone();
        p2p_config.max_num_inbound_peers = self.node.p2p.inbound_peers;
        p2p_config.max_num_outbound_peers = self.node.p2p.outbound_peers;
        p2p_config.root_dir = Some(self.node.data_dir.clone());
        p2p_config.addr_book = String::from(Path::new(&self.node.data_dir).join("addrbook.json").to_string_lossy());

        Ok(p2p_config)
    }

    pub fn get_db_info() {
        todo!()
    }

    pub fn get_tx_pool_config() {
        todo!()
    }

    pub fn get_genesis_config() {
        todo!()
    }

    pub fn get_mainchain_config() {
        todo!()
    }

    pub fn get_node_config() {
        
    }

    pub fn get_fast_sync_config() {
        todo!()
    }

    pub fn get_gas_oracle_config() {
        todo!()
    }

    pub fn new_log() {
        todo!()
    }

    pub fn get_consensus_config() {
        todo!()
    }

    pub fn get_consensus_params() {
        todo!()
    }

    pub fn get_chain_config() {
        todo!()
    }

    pub fn start() {
        todo!()
    }

    pub fn start_debug() {
        todo!()
    }
}

#[warn(unreachable_code)]
pub fn default_p2p_config() -> P2PConfig{

    P2PConfig {
        private_key: None, 
        root_dir: None, 
        listen_address: "tcp://0.0.0.0:26656".to_string(), 
        external_address: "".to_string(), 
        seeds: None, 
        persistent_peers: None, 
        upnp: false, 
        addr_book_strict: true, 
        max_num_inbound_peers: 40, 
        max_num_outbound_peers: 15, 
        unconditional_peer_ids: None, 
        persistent_peers_max_dial_period: Duration::from_secs(0),
        flush_throttle_timeout: Duration::from_millis(100),
        max_packet_msg_payload_size: 1024,
        send_rate: 5120000, // 5 mB/s
        recv_rate: 5120000, // 5 mB/s
        pex_reactor: true,
        seed_mode: false,
        private_peer_ids: None,
        allow_duplicate_ip: false,
        handshake_timeout: Duration::from_secs(20),
        dial_timeout: Duration::from_secs(3),
        test_dial_fail: false,
        test_fuzz: false,
        addr_book: String::from(Path::new(&default_data_dir()).join(DEFAULT_ADDR_BOOK_NAME).to_string_lossy())
    }
}

// DefaultDataDir is the default data directory to use for the databases and other
// persistence requirements.
pub fn default_data_dir() -> String {
    todo!()
}

// DefaultFuzzConnConfig returns the default config.
pub fn default_fuzz_conn_config() -> String {
    todo!()
}