pub mod blockchain {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.blockchain.rs"));
}

pub mod consensus {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.consensus.rs"));
}

pub mod crypto {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.crypto.rs"));
}

pub mod evidence {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.evidence.rs"));
}

pub mod p2p {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.p2p.rs"));
}

pub mod state {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.state.rs"));
}

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.types.rs"));
}
