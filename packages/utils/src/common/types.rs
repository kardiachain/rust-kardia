pub struct Config {
    pub node: Node,
}

pub struct Node {
    pub p2p: P2P,
    
    pub name : &str,
    pub data_dir: &str,
    pub http_host: &str,
    pub http_port: i32,
    pub http_modules: [&str],
    pub http_virtual_hosts: [&str],
    pub http_cors: [&str],
}

pub struct P2P {
        pub listen_address : &str,
        pub private_key : &str,
        pub inbound_peers: i32,
        pub outbound_peers: i32
}

impl Config {
    pub fn get_p2p_config() {
        todo!()
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
        todo!()
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